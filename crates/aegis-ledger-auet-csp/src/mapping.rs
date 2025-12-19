// crates/aegis-ledger-auet-csp/src/mapping.rs
use serde::{Serialize, Deserialize};

pub const DALN: u32 = 9;
pub const C_E: f64 = 1e-12;
pub const C_S: f64 = 5e-13;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MappingParams {
    pub d_src: u32,
    pub d_aln: u32,
    pub c_e: f64,
    pub c_s: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MappedAmounts {
    pub auet_units: u128,
    pub csp_units: u128,
}

fn to_u128_floor(x: f64) -> u128 {
    if !x.is_finite() || x < 0.0 {
        0
    } else {
        x.floor() as u128
    }
}

pub fn map_source_to_internal(balance_min_units: u128, params: &MappingParams) -> MappedAmounts {
    let b = balance_min_units as f64;
    let src_scale = 10f64.powi(params.d_src as i32);
    let aln_scale = 10f64.powi(params.d_aln as i32);

    let a_src = b / src_scale;
    let a_e = a_src * params.c_e;
    let a_s = a_src * params.c_s;

    let be = to_u128_floor(a_e * aln_scale);
    let bs = to_u128_floor(a_s * aln_scale);

    MappedAmounts {
        auet_units: be,
        csp_units: bs,
    }
}
