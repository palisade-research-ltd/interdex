// ============================================================================
// VPIN TRADING SIGNAL GENERATOR - PARAMETER CONFIGURATION
// ============================================================================

// === PARAMETER SET 1: CONSERVATIVE (Default) ===
// For stable markets, less noise, slower signals
// const VOLUME_PER_BAR: f64 = 300.0;
// const WINDOW_SIZE: usize = 15;
// const BATCH_SIZE: usize = 10;
// const DATA_LIMIT_MINUTES: usize = 4;
// const TOXICITY_THRESHOLD: f64 = 0.75;
// const MOMENTUM_WINDOW: usize = 5;
// const TARGET_BARS_PER_DAY: f64 = 1200.0;

// === PARAMETER SET 2: RESPONSIVE (Active) ===
// For volatile markets, faster signals, more sensitive
// const VOLUME_PER_BAR: f64 = 50.0;           // Smaller bars = faster completion
// const WINDOW_SIZE: usize = 8;               // Smaller window = more responsive
// const BATCH_SIZE: usize = 2;                // Smaller batches = more frequent updates
// const DATA_LIMIT_MINUTES: usize = 2;        // Less historical data
// const TOXICITY_THRESHOLD: f64 = 0.65;       // Lower threshold = earlier alerts
// const MOMENTUM_WINDOW: usize = 3;           // Shorter momentum window
// const TARGET_BARS_PER_DAY: f64 = 2000.0;    // More bars per day

// === PARAMETER SET 3: ULTRA-FAST (Scalping) ===
// For high-frequency trading, maximum sensitivity
const VOLUME_PER_BAR: f64 = 100.0;
const WINDOW_SIZE: usize = 8;
const BATCH_SIZE: usize = 4;
const DATA_LIMIT_MINUTES: usize = 1;
const TOXICITY_THRESHOLD: f64 = 0.5;
const MOMENTUM_WINDOW: usize = 4;
const TARGET_BARS_PER_DAY: f64 = 7000.0;

// === SIGNAL GENERATION PARAMETERS ===
const HIGH_MOMENTUM_SELL_THRESHOLD: f64 = 0.025; // Strong sell signal momentum
const MEDIUM_MOMENTUM_SELL_THRESHOLD: f64 = 0.015; // Medium sell signal momentum
const HIGH_MOMENTUM_BUY_THRESHOLD: f64 = 0.025; // Strong buy signal momentum
const MEDIUM_MOMENTUM_BUY_THRESHOLD: f64 = 0.015; // Medium buy signal momentum
const WEAK_MOMENTUM_THRESHOLD: f64 = 0.008; // Weak signal threshold

const HIGH_VPIN_SELL_THRESHOLD: f64 = 0.8; // VPIN level for strong sell
const LOW_VPIN_BUY_THRESHOLD: f64 = 0.2; // VPIN level for strong buy
const HIGH_TOXICITY_LEVEL: f64 = 0.85; // High toxicity classification
const MEDIUM_TOXICITY_LEVEL: f64 = 0.65; // Medium toxicity classification

// === SYSTEM PARAMETERS ===
const MAX_CONSECUTIVE_ERRORS: usize = 20; // Max errors before reset
const ERROR_SLEEP_SECONDS: u64 = 3; // Sleep duration on error
const RESET_SLEEP_SECONDS: u64 = 30; // Sleep duration on reset
const HEALTH_CHECK_INTERVAL: usize = 120; // Health check every N iterations
const DETAILED_ANALYSIS_INTERVAL: usize = 10; // Detailed logging interval
const MAX_VPIN_HISTORY: usize = 20; // Max VPIN values to keep

// === TRADING STATISTICS (Symbol-specific estimates) ===
const SOLUSDT_DAILY_VOLUME: f64 = 65_000_000.0;
const SOLUSDT_TRADES_PER_MINUTE: f64 = 45.0;
const BTCUSDT_DAILY_VOLUME: f64 = 60_000_000.0;
const BTCUSDT_TRADES_PER_MINUTE: f64 = 90.0;
const DEFAULT_DAILY_VOLUME: f64 = 6_000_000.0;
const DEFAULT_TRADES_PER_MINUTE: f64 = 25.0;

// ============================================================================
// IMPLEMENTATION CODE
// ============================================================================

use atelier_quant::data::VpinResult;
use atelier_quant::vpin::VpinStatistics;

use atelier_quant::VpinCalculator;
use chrono::{DateTime, Utc};
use ix_execution::{ClickHouseClient, queries};
use std::env;
use tokio::time::{Duration, Instant, sleep};

#[derive(Debug, Clone)]
struct VpinParameters {
    volume_per_bar: f64,
    window_size: usize,
    batch_size: usize,
    limit: String,
    toxicity_threshold: f64,
    momentum_window: usize,
}

impl VpinParameters {
    async fn optimize_for_symbol(
        symbol: &str,
        exchange: &str,
        _client: &ClickHouseClient,
    ) -> anyhow::Result<Self> {
        println!("🔧 Optimizing parameters for {}/{}", exchange, symbol);

        let stats = get_recent_trading_stats(symbol, exchange).await?;

        // Use configurable parameters instead of hardcoded values
        let volume_per_bar = (stats.avg_daily_volume / TARGET_BARS_PER_DAY)
            .max(VOLUME_PER_BAR * 0.5) // Min 50% of configured value
            .min(VOLUME_PER_BAR * 2.0); // Max 200% of configured value

        // Use the configured window size with minor adjustments based on activity
        let window_size = match stats.avg_trades_per_minute {
            0.0..=30.0 => WINDOW_SIZE + 2,      // Add 2 for low activity
            30.0..=80.0 => WINDOW_SIZE,         // Use configured value
            _ => WINDOW_SIZE.saturating_sub(2), // Subtract 2 for high activity
        };

        let trades_per_second = stats.avg_trades_per_minute / 60.0;
        let batch_size = (trades_per_second * 2.0)
            .max(BATCH_SIZE as f64 * 0.5)
            .min(BATCH_SIZE as f64 * 2.0) as usize;

        let limit = (stats.avg_trades_per_minute * DATA_LIMIT_MINUTES as f64) as usize;

        println!("📊 Calculated parameters:");
        println!(
            "   - Volume per bar: {:.1} (configured: {})",
            volume_per_bar, VOLUME_PER_BAR
        );
        println!(
            "   - Window size: {} bars (configured: {})",
            window_size, WINDOW_SIZE
        );
        println!(
            "   - Batch size: {} trades (configured: {})",
            batch_size, BATCH_SIZE
        );
        println!(
            "   - Data limit: {} trades ({} min)",
            limit, DATA_LIMIT_MINUTES
        );
        println!("   - Toxicity threshold: {}", TOXICITY_THRESHOLD);
        println!("   - Momentum window: {}", MOMENTUM_WINDOW);

        Ok(VpinParameters {
            volume_per_bar,
            window_size,
            batch_size,
            limit: limit.to_string(),
            toxicity_threshold: TOXICITY_THRESHOLD,
            momentum_window: MOMENTUM_WINDOW,
        })
    }
}

#[derive(Debug)]
struct TradingStats {
    avg_daily_volume: f64,
    avg_trades_per_minute: f64,
    recent_volatility: f64,
}

async fn get_recent_trading_stats(
    symbol: &str,
    _exchange: &str,
) -> anyhow::Result<TradingStats> {
    let (avg_daily_volume, avg_trades_per_minute) = match symbol {
        "SOLUSDT" => (SOLUSDT_DAILY_VOLUME, SOLUSDT_TRADES_PER_MINUTE),
        "BTCUSDT" => (BTCUSDT_DAILY_VOLUME, BTCUSDT_TRADES_PER_MINUTE),
        _ => (DEFAULT_DAILY_VOLUME, DEFAULT_TRADES_PER_MINUTE),
    };

    Ok(TradingStats {
        avg_daily_volume,
        avg_trades_per_minute,
        recent_volatility: 0.03,
    })
}

#[derive(Debug, Clone)]
struct TradeFlowSignal {
    timestamp: DateTime<Utc>,
    signal: String,
    strength: f64,
    confidence: String,
    vpin_value: f64,
    vpin_momentum: f64,
    toxicity_level: String,
    bars_in_calculation: usize,
    error_status: String,
}

impl TradeFlowSignal {
    fn generate(
        vpin_value: f64,
        vpin_momentum: f64,
        is_toxic: bool,
        bars_count: usize,
        has_error: bool,
    ) -> Self {
        let timestamp = Utc::now();

        if has_error {
            return TradeFlowSignal {
                timestamp,
                signal: "NEUTRAL".to_string(),
                strength: 0.0,
                confidence: "ERROR".to_string(),
                vpin_value,
                vpin_momentum: 0.0,
                toxicity_level: "UNKNOWN".to_string(),
                bars_in_calculation: bars_count,
                error_status: "ERROR_FALLBACK".to_string(),
            };
        }

        // Use configurable thresholds for signal generation
        let (signal, confidence) = match (vpin_momentum.abs(), is_toxic, vpin_value) {
            // Strong sell signals
            (momentum, true, vpin)
                if momentum > HIGH_MOMENTUM_SELL_THRESHOLD
                    && vpin > HIGH_VPIN_SELL_THRESHOLD =>
            {
                ("SELL", "HIGH")
            }
            (momentum, true, _) if momentum > MEDIUM_MOMENTUM_SELL_THRESHOLD => {
                ("SELL", "MEDIUM")
            }

            // Strong buy signals
            (momentum, false, vpin)
                if momentum > HIGH_MOMENTUM_BUY_THRESHOLD
                    && vpin < LOW_VPIN_BUY_THRESHOLD =>
            {
                ("BUY", "HIGH")
            }
            (momentum, false, _) if momentum > MEDIUM_MOMENTUM_BUY_THRESHOLD => {
                ("BUY", "MEDIUM")
            }

            // Weak signals
            (momentum, _, _)
                if momentum > WEAK_MOMENTUM_THRESHOLD && vpin_momentum > 0.0 =>
            {
                ("SELL", "LOW")
            }
            (momentum, _, _)
                if momentum > WEAK_MOMENTUM_THRESHOLD && vpin_momentum < 0.0 =>
            {
                ("BUY", "LOW")
            }

            _ => ("NEUTRAL", "LOW"),
        };

        let strength = (vpin_momentum.abs() * 10.0).min(1.0);

        let toxicity_level = match vpin_value {
            v if v > HIGH_TOXICITY_LEVEL => "HIGH",
            v if v > MEDIUM_TOXICITY_LEVEL => "MEDIUM",
            _ => "LOW",
        };

        TradeFlowSignal {
            timestamp,
            signal: signal.to_string(),
            strength,
            confidence: confidence.to_string(),
            vpin_value,
            vpin_momentum,
            toxicity_level: toxicity_level.to_string(),
            bars_in_calculation: bars_count,
            error_status: "OK".to_string(),
        }
    }

    fn should_alert(&self) -> bool {
        self.error_status == "OK"
            && match self.confidence.as_str() {
                "HIGH" => true,
                "MEDIUM" => self.strength > 0.8,
                _ => false,
            }
    }
}

struct UltraSafeVpinCalculator {
    calculator: VpinCalculator,
    last_known_vpin: f64,
    consecutive_errors: usize,
    vpin_history: Vec<f64>,
    max_history: usize,
}

impl UltraSafeVpinCalculator {
    fn new(
        volume_per_bar: f64,
        window_size: usize,
        dist_p0: f64,
    ) -> anyhow::Result<Self> {
        let calculator = VpinCalculator::new(volume_per_bar, window_size, dist_p0)?;
        Ok(UltraSafeVpinCalculator {
            calculator,
            last_known_vpin: 0.0,
            consecutive_errors: 0,
            vpin_history: Vec::new(),
            max_history: MAX_VPIN_HISTORY,
        })
    }

    fn safe_process_trades<T>(&mut self, trades: &[T]) -> (Option<VpinResult>, bool)
    where
        T: atelier_quant::data::HasTradeFields,
    {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.calculator.process_trades(trades)
        })) {
            Ok(Ok(result)) => {
                self.consecutive_errors = 0;
                if let Some(ref vpin_result) = result {
                    self.last_known_vpin = vpin_result.vpin;
                    self.vpin_history.push(vpin_result.vpin);
                    if self.vpin_history.len() > self.max_history {
                        self.vpin_history.remove(0);
                    }
                }
                (result, false)
            }
            Ok(Err(e)) => {
                self.consecutive_errors += 1;
                eprintln!("⚠️ VPIN calculation error: {:?}", e);
                (None, true)
            }
            Err(_) => {
                self.consecutive_errors += 1;
                eprintln!("⚠️ VPIN calculation panicked");
                (None, true)
            }
        }
    }

    fn safe_is_toxic(&mut self, threshold: f64) -> (bool, bool) {
        (self.last_known_vpin > threshold, false)
    }

    fn safe_calculate_momentum(&self, window: usize) -> (f64, bool) {
        if self.vpin_history.len() < 5 {
            return (0.0, false);
        }

        let safe_window = window.min(self.vpin_history.len() - 1).max(2);

        if safe_window >= self.vpin_history.len() {
            return (0.0, false);
        }

        let recent_vpins = &self.vpin_history[self.vpin_history.len() - safe_window..];

        if recent_vpins.len() < 3 {
            return (0.0, false);
        }

        let momentum =
            (recent_vpins[recent_vpins.len() - 1] - recent_vpins[0]) / safe_window as f64;
        (momentum, false)
    }

    fn safe_get_statistics(&mut self) -> Option<VpinStatistics> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.calculator.get_statistics()
        })) {
            Ok(stats) => Some(stats),
            Err(_) => {
                eprintln!("⚠️ Statistics calculation panicked");
                None
            }
        }
    }

    fn get_bars_count(&self) -> usize {
        self.vpin_history.len().min(15)
    }

    fn get_volume_bars(&mut self) -> Vec<atelier_quant::VolumeBar> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.calculator.get_volume_bars()
        })) {
            Ok(bars) => bars,
            Err(_) => {
                eprintln!("⚠️ Get volume bars panicked");
                Vec::new()
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Starting Ultra-Safe Real-Time VPIN Trading Signal Generator");
    println!("===============================================================");
    println!("📋 Current Parameter Set: RESPONSIVE");
    println!(
        "   Volume/Bar: {} | Window: {} | Batch: {} | Toxicity: {}",
        VOLUME_PER_BAR, WINDOW_SIZE, BATCH_SIZE, TOXICITY_THRESHOLD
    );
    println!();

    let ch_url = env::var("CLICKHOUSE_URL")
        .unwrap_or_else(|_| "http://localhost:8123".to_string());
    let ch_db = env::var("CLICKHOUSE_DB").unwrap_or_else(|_| "operations".to_string());

    let ch_pt_client = ClickHouseClient::builder()
        .url(ch_url.clone())
        .database(ch_db.clone())
        .build()
        .await?;

    let p_exchange = env::var("EXCHANGE").unwrap_or_else(|_| "Bybit".to_string());
    let p_symbol = env::var("SYMBOL").unwrap_or_else(|_| "SOLUSDT".to_string());

    println!("📈 Trading Pair: {}/{}", p_exchange, p_symbol);
    println!("🔗 ClickHouse: {} (database: {})", ch_url, ch_db);

    let params =
        VpinParameters::optimize_for_symbol(&p_symbol, &p_exchange, &ch_pt_client)
            .await?;

    let mut ultra_safe_vpin_calculator =
        UltraSafeVpinCalculator::new(params.volume_per_bar, params.window_size, 0.25)?;

    println!("✅ Ultra-Safe VPIN Calculator initialized");
    println!();

    let mut iteration = 0;
    let mut last_vpin = 0.0;
    let mut consecutive_errors = 0;
    let mut consecutive_high_vpin = 0;
    let start_time = Instant::now();

    loop {
        iteration += 1;
        let loop_start = Instant::now();

        if consecutive_errors > MAX_CONSECUTIVE_ERRORS {
            println!("⚠️ Too many errors, resetting and waiting...");
            sleep(Duration::from_secs(RESET_SLEEP_SECONDS)).await;
            consecutive_errors = 0;
            continue;
        }

        match queries::trades::read_tables::q_read_trades(
            p_exchange.clone(),
            p_symbol.clone(),
            params.limit.clone(),
        )
        .await
        {
            Ok(pt_read_query) => {
                match ch_pt_client
                    .read_table::<queries::trades::ClickhouseTradeData>(&pt_read_query)
                    .await
                {
                    Ok(trades) => {
                        consecutive_errors = 0;

                        if !trades.is_empty() {
                            let mut batch_processed = false;

                            for batch in trades.chunks(params.batch_size) {
                                let (vpin_result, had_error) =
                                    ultra_safe_vpin_calculator.safe_process_trades(batch);

                                match vpin_result {
                                    Some(vpin_result) => {
                                        batch_processed = true;

                                        let (is_toxic, toxicity_error) =
                                            ultra_safe_vpin_calculator
                                                .safe_is_toxic(params.toxicity_threshold);

                                        let bars_count =
                                            ultra_safe_vpin_calculator.get_bars_count();
                                        let (momentum, momentum_error) =
                                            if bars_count >= 8 {
                                                ultra_safe_vpin_calculator
                                                    .safe_calculate_momentum(
                                                        params.momentum_window,
                                                    )
                                            } else {
                                                (0.0, false)
                                            };

                                        let has_any_error =
                                            had_error || toxicity_error || momentum_error;

                                        let trade_signal = TradeFlowSignal::generate(
                                            vpin_result.vpin,
                                            momentum,
                                            is_toxic,
                                            bars_count,
                                            has_any_error,
                                        );

                                        // Track consecutive high VPIN
                                        if vpin_result.vpin >= 0.8 {
                                            consecutive_high_vpin += 1;
                                        } else {
                                            consecutive_high_vpin = 0;
                                        }

                                        let status_icon =
                                            if has_any_error { "⚠️" } else { "⏰" };
                                        println!(
                                            "{} {} | #{:04} | VPIN: {:.4} | Momentum: {:+.4} | 📊 {}: {} ({:.2}) | 🧪 {} | Bars: {} | Hist: {}{}",
                                            status_icon,
                                            Utc::now().format("%H:%M:%S"),
                                            iteration,
                                            vpin_result.vpin,
                                            momentum,
                                            trade_signal.signal,
                                            trade_signal.confidence,
                                            trade_signal.strength,
                                            trade_signal.toxicity_level,
                                            vpin_result.num_bars,
                                            ultra_safe_vpin_calculator.vpin_history.len(),
                                            if consecutive_high_vpin > 10 {
                                                " 🔴"
                                            } else {
                                                ""
                                            }
                                        );

                                        // Detailed analysis every N iterations
                                        if iteration % DETAILED_ANALYSIS_INTERVAL == 0 {
                                            let volume_bars = ultra_safe_vpin_calculator
                                                .get_volume_bars();
                                            if volume_bars.len() >= 3 {
                                                println!("🔍 VPIN Analysis:");
                                                let recent_bars = &volume_bars
                                                    [volume_bars
                                                        .len()
                                                        .saturating_sub(3)..];
                                                for (i, bar) in
                                                    recent_bars.iter().enumerate()
                                                {
                                                    let imbalance_ratio =
                                                        bar.order_imbalance / bar.volume;
                                                    println!(
                                                        "   Bar -{}: Vol={:.1}, OI={:.4}, Ratio={:.4}, BuyProb={:.3}",
                                                        2 - i,
                                                        bar.volume,
                                                        bar.order_imbalance,
                                                        imbalance_ratio,
                                                        bar.buy_probability
                                                    );
                                                }
                                                let avg_imbalance_ratio = volume_bars
                                                    .iter()
                                                    .map(|bar| {
                                                        bar.order_imbalance / bar.volume
                                                    })
                                                    .sum::<f64>()
                                                    / volume_bars.len() as f64;
                                                println!(
                                                    "   Avg imbalance ratio: {:.4} | Consecutive high VPIN: {}",
                                                    avg_imbalance_ratio,
                                                    consecutive_high_vpin
                                                );
                                            }
                                        }

                                        if trade_signal.should_alert() {
                                            println!(
                                                "🚨 TRADING ALERT: {} signal with {} confidence (strength: {:.2})",
                                                trade_signal.signal,
                                                trade_signal.confidence,
                                                trade_signal.strength
                                            );
                                        }

                                        if consecutive_high_vpin > 20 {
                                            println!(
                                                "🔴 ALERT: VPIN stuck at high level for {} iterations",
                                                consecutive_high_vpin
                                            );
                                        }

                                        last_vpin = vpin_result.vpin;
                                    }
                                    None => {
                                        if iteration % 15 == 0 {
                                            if let Some(stats) =
                                                ultra_safe_vpin_calculator
                                                    .safe_get_statistics()
                                            {
                                                println!(
                                                    "⏳ #{:04} | Building bars... ({:.1}/{:.1}) | Last VPIN: {:.4} | Hist: {}",
                                                    iteration,
                                                    stats.current_bar_volume,
                                                    params.volume_per_bar,
                                                    last_vpin,
                                                    ultra_safe_vpin_calculator
                                                        .vpin_history
                                                        .len()
                                                );
                                            } else {
                                                println!(
                                                    "⏳ #{:04} | Processing... | Last VPIN: {:.4}",
                                                    iteration, last_vpin
                                                );
                                            }
                                        }
                                    }
                                }

                                if batch_processed {
                                    break;
                                }
                            }
                        } else {
                            if iteration % 60 == 0 {
                                println!("📭 No trades (iteration {})", iteration);
                            }
                        }
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        if consecutive_errors % 5 == 1 {
                            println!("❌ DB error #{}: {:?}", consecutive_errors, e);
                        }
                        sleep(Duration::from_secs(ERROR_SLEEP_SECONDS)).await;
                    }
                }
            }
            Err(e) => {
                consecutive_errors += 1;
                if consecutive_errors % 5 == 1 {
                    println!("❌ Query error #{}: {:?}", consecutive_errors, e);
                }
                sleep(Duration::from_secs(ERROR_SLEEP_SECONDS)).await;
            }
        }

        let loop_duration = loop_start.elapsed();
        if loop_duration < Duration::from_secs(1) {
            sleep(Duration::from_secs(1) - loop_duration).await;
        }

        if iteration % HEALTH_CHECK_INTERVAL == 0 {
            let uptime = start_time.elapsed().as_secs();
            println!(
                "💚 Health: Iter {}, Up: {}s, Errs: {}, VPIN: {:.4}, Hist: {}, HighVPIN: {}",
                iteration,
                uptime,
                consecutive_errors,
                last_vpin,
                ultra_safe_vpin_calculator.vpin_history.len(),
                consecutive_high_vpin
            );
        }
    }
}
