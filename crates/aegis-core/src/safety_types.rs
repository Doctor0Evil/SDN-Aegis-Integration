use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadEnvelope {
    pub d_ion: u64,
    pub sar_mw_per_kg: u32,
    pub j_ma_per_m2: u32,
    pub d_ion_max: u64,
    pub sar_max_mw_per_kg: u32,
    pub j_max_ma_per_m2: u32,
}

impl RadEnvelope {
    pub fn can_apply(&self, dd_ion: u64, d_sar: u32, d_j: u32) -> bool {
        self.d_ion.saturating_add(dd_ion) <= self.d_ion_max &&
        self.sar_mw_per_kg.saturating_add(d_sar) <= self.sar_max_mw_per_kg &&
        self.j_ma_per_m2.saturating_add(d_j) <= self.j_max_ma_per_m2
    }

    pub fn apply(&mut self, dd_ion: u64, d_sar: u32, d_j: u32) {
        self.d_ion = self.d_ion.saturating_add(dd_ion).min(self.d_ion_max);
        self.sar_mw_per_kg = self.sar_mw_per_kg.saturating_add(d_sar).min(self.sar_max_mw_per_kg);
        self.j_ma_per_m2 = self.j_ma_per_m2.saturating_add(d_j).min(self.j_max_ma_per_m2);
    }
}