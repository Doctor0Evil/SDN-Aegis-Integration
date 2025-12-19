pub mod energy_state;
pub mod safety_types;
pub mod flow_id;
pub mod hex_hash;

pub use energy_state::*;
pub use safety_types::*;
pub use flow_id::*;
pub use hex_hash::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hex_smoke() {
        let s = sha256_hex(b"abc");
        assert!(s.starts_with("0x"));
    }
}
