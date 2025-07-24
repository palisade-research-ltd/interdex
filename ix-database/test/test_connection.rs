#![cfg(test)]

mod tests {

    use ix_database::queries::orderbooks::create_tables::create_orderbooks_table_ddl;
    use ix_database::ClickHouseClient;

    #[tokio::test]
    async fn test_connection_lifecycle() {
        let client = ClickHouseClient::builder()
            .url("http://localhost:8123")
            .database("default")
            .build()
            .await
            .expect("Failed to create client");

        let connection_id = client
            .create_connection()
            .await
            .expect("Failed to create connection");

        let connections = client
            .get_connections()
            .await
            .expect("Failed to get connections");

        assert_eq!(connections.len(), 1);

        client
            .destroy_connection(connection_id)
            .await
            .expect("Failed to destroy connection");

        let connections = client
            .get_connections()
            .await
            .expect("Failed to get connections");

        assert_eq!(connections.len(), 0);
    }

    #[tokio::test]
    async fn test_client_builder() {
        let client = ClickHouseClient::builder()
            .url("http://localhost:8123")
            .database("test")
            .build()
            .await;

        assert!(client.is_ok());
    }

    #[test]
    fn test_create_table_ddl() {
        let ddl = create_orderbooks_table_ddl();
        assert!(ddl.contains("CREATE TABLE"));
        assert!(ddl.contains("orderbooks"));
        assert!(ddl.contains("MergeTree"));
    }

    #[test]
    fn test_read_orderbook() {}

    #[test]
    fn test_insert_orderbook() {}
}
