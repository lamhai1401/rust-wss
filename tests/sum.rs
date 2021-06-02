extern crate wss;
use wss::utils::rd::sum;

#[test]
fn sum_test() {
    assert_eq!(sum(6, 8), 14);
}
