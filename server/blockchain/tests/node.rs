use rand::Rng;

#[path = "../src/node.rs"]
mod node;

#[test]
fn generate_random_number_always_returns_number_between_0_and_max() {
    let max = 10;
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let num = rng.gen_range(0..max);
        assert!(num >= 0 && num < max);
    }
}

#[test]
fn test_get_index() {
    let weights = vec![3, 2, 1];
    let random_number = node::generate_random_number(6);
    let index = node::get_index(weights, random_number);
    assert_eq!(index, 2);
}

#[test]
fn test_sort_by_weight() {
    let weights = vec![1, 3, 2];
    let expected_weights = vec![3, 2, 1];

    let sorted_weights = node::get_sorted_weights(weights);
    assert_eq!(sorted_weights, expected_weights);
}
