#[path = "../src/arg.rs"]
mod arg;

#[test]
fn get_node_name_returns_camper() {
    assert_eq!(arg::get_node_name(), "Camper");
}
