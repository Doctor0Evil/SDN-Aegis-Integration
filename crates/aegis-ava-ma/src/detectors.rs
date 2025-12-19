pub fn detect_anomaly(latency_ms: u64, jitter_ms: u64, drop_rate_ppm: u32) -> bool {
    (latency_ms > 200) || (jitter_ms > 50) || (drop_rate_ppm > 1000)
}
