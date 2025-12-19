pub fn risk_score(latency_ms: u64, jitter_ms: u64, drop_rate_ppm: u32) -> f64 {
    let mut r = 0.0;
    if latency_ms > 100 { r += (latency_ms as f64 - 100.0) / 100.0; }
    if jitter_ms > 10 { r += (jitter_ms as f64 - 10.0) / 50.0; }
    r += (drop_rate_ppm as f64) / 1_000_000.0;
    r
}
