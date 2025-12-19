// crates/aegis-core/src/energy_state.rs
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergyVector {
    pub auet: u128,      // AU.ET units
    pub csp: u128,       // CSP units
    pub e_compute: u128, // Ea0
    pub e_bio: u128,     // Ea1
    pub e_risk: u128,    // Ea2
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DualPool {
    pub fast: u128,
    pub slow: u128,
}

impl DualPool {
    pub fn step(&self, drain_fast: u128, drain_slow: u128,
                regen_fast: u128, regen_slow: u128) -> DualPool {
        use std::cmp::min;
        let nf = self.fast.saturating_sub(drain_fast).saturating_add(regen_fast);
        let ns = self.slow.saturating_sub(drain_slow).saturating_add(regen_slow);
        DualPool { fast: nf, slow: ns }
    }
}
