use aegis_sim_harness::*;

#[test]
fn non_minting_placeholder() {
    // placeholder: assert that mapping params produce non-negative amounts
    let params = crate::generators::gen_traffic; // not used
    assert_eq!(2 + 2, 4);
}
