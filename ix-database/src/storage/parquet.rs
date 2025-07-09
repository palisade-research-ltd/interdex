//! Parquet storage functionality for OrderBook data
//!
//! This module provides efficient storage and retrieval of OrderBook data
//! using Apache Parquet format with partitioning by exchange, date, and time.

// use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, StringArray, TimestampNanosecondArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Utc};
use parquet::arrow::{
    arrow_reader::{
        ArrowReaderBuilder, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder,
    },
    ArrowWriter,
};
use parquet::file::properties::WriterProperties;
use serde::{Deserialize, Serialize};
use tokio::fs;
use ix_cex::models::orderbook::{PriceLevel, OrderBook};

use crate::{DatabaseError, DatabaseResult};

/// Partitioning strategy for Parquet files
#[derive(Debug, Clone)]
pub enum PartitionStrategy {
    /// Partition by exchange/day/hour/minute
    ExchangeDateTime,
    /// Partition by symbol/exchange/day
    SymbolExchangeDay,
    /// Partition by day only
    DayOnly,
    /// No partitioning
    None,
}

/// Parquet storage manager
pub struct ParquetStorage {
    base_path: PathBuf,
    partition_strategy: PartitionStrategy,
    writer_properties: WriterProperties,
}

impl ParquetStorage {
    /// Create a new Parquet storage manager
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        partition_strategy: PartitionStrategy,
    ) -> Self {
        let writer_properties = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .set_max_row_group_size(1000000)
            .set_data_page_size_limit(1024 * 1024) // 1MB
            .build();

        Self {
            base_path: base_path.as_ref().to_path_buf(),
            partition_strategy,
            writer_properties,
        }
    }

    /// Write OrderBook records to Parquet file
    pub async fn write_orderbooks(
        &self,
        orderbooks: &[OrderBook],
    ) -> DatabaseResult<String> {
        if orderbooks.is_empty() {
            return Err(DatabaseError::OperationFailed(
                "No orderbooks to write".to_string(),
            ));
        }

        let file_path = self.generate_file_path(&orderbooks[0]);
        self.ensure_directory_exists(&file_path).await?;

        let schema = Self::create_orderbook_schema();
        let record_batch = Self::create_record_batch(&schema, orderbooks)?;

        let file = std::fs::File::create(&file_path).map_err(DatabaseError::IoError)?;

        let mut writer =
            ArrowWriter::try_new(file, schema, Some(self.writer_properties.clone()))
                .map_err(|e| DatabaseError::ParquetError(e.to_string()))?;

        writer
            .write(&record_batch)
            .map_err(|e| DatabaseError::ParquetError(e.to_string()))?;

        writer
            .close()
            .map_err(|e| DatabaseError::ParquetError(e.to_string()))?;

        Ok(file_path.to_string_lossy().to_string())
    }

    /// Read OrderBook records from Parquet file
    pub async fn read_orderbooks<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> DatabaseResult<Vec<OrderBook>> {
        let file =
            std::fs::File::open(file_path.as_ref()).map_err(DatabaseError::IoError)?;

        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

        let reader: ParquetRecordBatchReader = builder.build().unwrap();

        let mut orderbooks = Vec::new();
        for batch in reader {
            let batch = batch.map_err(|e| DatabaseError::ParquetError(e.to_string()))?;

            let parsed_orderbooks = Self::parse_record_batch(&batch)?;
            orderbooks.extend(parsed_orderbooks);
        }

        Ok(orderbooks)
    }

    /// List all Parquet files in the storage
    pub async fn list_files(&self) -> DatabaseResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.collect_parquet_files(&self.base_path, &mut files)
            .await?;
        Ok(files)
    }

    /// Get file statistics
    pub async fn get_file_stats<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> DatabaseResult<FileStats> {
        let metadata = fs::metadata(file_path.as_ref())
            .await
            .map_err(DatabaseError::IoError)?;

        let file =
            std::fs::File::open(file_path.as_ref()).map_err(DatabaseError::IoError)?;

        let reader = ArrowReaderBuilder::try_new(file)
            .map_err(|e| DatabaseError::ParquetError(e.to_string()))?;

        let parquet_metadata = reader.metadata();
        let row_count = parquet_metadata.file_metadata().num_rows();
        let row_groups = parquet_metadata.num_row_groups();

        Ok(FileStats {
            file_path: file_path.as_ref().to_path_buf(),
            file_size: metadata.len(),
            row_count: row_count as u64,
            row_groups: row_groups as u64,
            compression: "SNAPPY".to_string(),
            created_at: metadata
                .created()
                .unwrap_or(std::time::SystemTime::now())
                .into(),
        })
    }

    /// Clean up old files based on retention policy
    pub async fn cleanup_old_files(&self, retention_days: u32) -> DatabaseResult<u32> {
        let cutoff_time = Utc::now() - chrono::Duration::days(retention_days as i64);
        let mut deleted_count = 0;

        let files = self.list_files().await?;
        for file_path in files {
            let metadata = fs::metadata(&file_path)
                .await
                .map_err(DatabaseError::IoError)?;

            let created_time: DateTime<Utc> = metadata
                .created()
                .unwrap_or(std::time::SystemTime::now())
                .into();

            if created_time < cutoff_time {
                fs::remove_file(&file_path)
                    .await
                    .map_err(DatabaseError::IoError)?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    /// Generate file path based on partitioning strategy
    fn generate_file_path(&self, orderbook: &OrderBook) -> PathBuf {
        match self.partition_strategy {
            PartitionStrategy::ExchangeDateTime => {
                let date = orderbook.timestamp.format("%Y%m%d").to_string();
                let hour = orderbook.timestamp.format("%H").to_string();
                let minute = orderbook.timestamp.format("%M").to_string();

                self.base_path
                    .join(&orderbook.exchange)
                    .join(&date)
                    .join(&hour)
                    .join(&minute)
                    .join(format!("{}.parquet", orderbook.symbol))
            }
            PartitionStrategy::SymbolExchangeDay => {
                let date = orderbook.timestamp.format("%Y%m%d").to_string();

                self.base_path
                    .join(&orderbook.symbol)
                    .join(&orderbook.exchange)
                    .join(&date)
                    .join("data.parquet")
            }
            PartitionStrategy::DayOnly => {
                let date = orderbook.timestamp.format("%Y%m%d").to_string();

                self.base_path.join(&date).join(format!(
                    "{}_{}.parquet",
                    orderbook.exchange, orderbook.symbol
                ))
            }
            PartitionStrategy::None => self.base_path.join(format!(
                "{}_{}.parquet",
                orderbook.exchange, orderbook.symbol
            )),
        }
    }

    /// Create Arrow schema for OrderBook data
    fn create_orderbook_schema() -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("symbol", DataType::Utf8, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Nanosecond, None),
                false,
            ),
            Field::new("bids", DataType::Utf8, false), // JSON string
            Field::new("asks", DataType::Utf8, false), // JSON string
            Field::new("last_update_id", DataType::UInt64, false),
            Field::new("sequence", DataType::UInt64, true),
        ]))
    }

    /// Create record batch from OrderBook data
    fn create_record_batch(
        schema: &Arc<Schema>,
        orderbooks: &[OrderBook],
    ) -> DatabaseResult<RecordBatch> {
        let symbols: ArrayRef = Arc::new(StringArray::from(
            orderbooks
                .iter()
                .map(|ob| ob.symbol.clone())
                .collect::<Vec<_>>(),
        ));

        let exchanges: ArrayRef = Arc::new(StringArray::from(
            orderbooks
                .iter()
                .map(|ob| ob.exchange.clone())
                .collect::<Vec<_>>(),
        ));

        let timestamps: ArrayRef = Arc::new(TimestampNanosecondArray::from(
            orderbooks
                .iter()
                .map(|ob| ob.timestamp.timestamp_nanos_opt().unwrap_or(0))
                .collect::<Vec<_>>(),
        ));

        let bids: ArrayRef = Arc::new(StringArray::from(
            orderbooks
                .iter()
                .map(|ob| serde_json::to_string(&ob.bids).unwrap_or_default())
                .collect::<Vec<_>>(),
        ));

        let asks: ArrayRef = Arc::new(StringArray::from(
            orderbooks
                .iter()
                .map(|ob| serde_json::to_string(&ob.asks).unwrap_or_default())
                .collect::<Vec<_>>(),
        ));

        let last_update_ids: ArrayRef = Arc::new(UInt64Array::from(
            orderbooks
                .iter()
                .map(|ob| ob.last_update_id)
                .collect::<Vec<_>>(),
        ));

        let sequences: ArrayRef = Arc::new(UInt64Array::from(
            orderbooks
                .iter()
                .map(|ob| ob.sequence.unwrap_or(0))
                .collect::<Vec<_>>(),
        ));

        let arrays = vec![
            symbols,
            exchanges,
            timestamps,
            bids,
            asks,
            last_update_ids,
            sequences,
        ];

        RecordBatch::try_new(schema.clone(), arrays)
            .map_err(|e| DatabaseError::ParquetError(e.to_string()))
    }

    /// Parse record batch into OrderBook data
    fn parse_record_batch(batch: &RecordBatch) -> DatabaseResult<Vec<OrderBook>> {
        let mut orderbooks = Vec::new();

        let symbols = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid symbol column".to_string())
            })?;
        let exchanges = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid exchange column".to_string())
            })?;
        let timestamps = batch
            .column(2)
            .as_any()
            .downcast_ref::<TimestampNanosecondArray>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid timestamp column".to_string())
            })?;
        let bids = batch
            .column(3)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid bids column".to_string())
            })?;
        let asks = batch
            .column(4)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid asks column".to_string())
            })?;
        let last_update_ids = batch
            .column(5)
            .as_any()
            .downcast_ref::<UInt64Array>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid last_update_id column".to_string())
            })?;
        let sequences = batch
            .column(6)
            .as_any()
            .downcast_ref::<UInt64Array>()
            .ok_or_else(|| {
                DatabaseError::ParquetError("Invalid sequence column".to_string())
            })?;

        for i in 0..batch.num_rows() {
            let symbol = symbols.value(i).to_string();
            let exchange = exchanges.value(i).to_string();
            let timestamp = DateTime::from_timestamp_nanos(timestamps.value(i));
            let bids_json = bids.value(i);
            let asks_json = asks.value(i);
            let last_update_id = last_update_ids.value(i);
            let sequence = if sequences.is_null(i) {
                None
            } else {
                Some(sequences.value(i))
            };

            let bid_levels: Vec<PriceLevel> = serde_json::from_str(bids_json)
                .map_err(DatabaseError::SerializationError)?;
            let ask_levels: Vec<PriceLevel> = serde_json::from_str(asks_json)
                .map_err(DatabaseError::SerializationError)?;

            let orderbook = OrderBook::new(
                symbol,
                exchange,
                timestamp,

                bid_levels,
                ask_levels,
                Some(last_update_id),
                sequence,
            );

            orderbooks.push(orderbook);
        }

        Ok(orderbooks)
    }

    /// Ensure directory exists
    async fn ensure_directory_exists(&self, file_path: &Path) -> DatabaseResult<()> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(DatabaseError::IoError)?;
        }
        Ok(())
    }

    /// Recursively collect all Parquet files
    async fn collect_parquet_files(
        &self,
        dir: &Path,
        files: &mut Vec<PathBuf>,
    ) -> DatabaseResult<()> {
        let mut entries = fs::read_dir(dir).await.map_err(DatabaseError::IoError)?;

        while let Some(entry) =
            entries.next_entry().await.map_err(DatabaseError::IoError)?
        {
            let path = entry.path();
            if path.is_dir() {
                Box::pin(self.collect_parquet_files(&path, files)).await?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("parquet") {
                files.push(path);
            }
        }

        Ok(())
    }
}

/// File statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub file_path: PathBuf,
    pub file_size: u64,
    pub row_count: u64,
    pub row_groups: u64,
    pub compression: String,
    pub created_at: DateTime<Utc>,
}
