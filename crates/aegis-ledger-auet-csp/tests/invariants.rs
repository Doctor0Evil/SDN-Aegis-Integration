use aegis_ledger_auet_csp::mapping::*;

#[test]
fn map_is_monotone() {
    let p = MappingParams { d_src: 2, d_aln: 9, c_e: C_E, c_s: C_S };
    let m1 = map_source_to_internal(1000, &p);
    let m2 = map_source_to_internal(2000, &p);
    assert!(m2.auet_units >= m1.auet_units);
    assert!(m2.csp_units >= m1.csp_units);
}
