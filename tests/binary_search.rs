extern crate cauly_rust_leetcode_utils;
use cauly_rust_leetcode_utils::binary_search::*;

#[test]
fn test1() {
    let result = BinarySearch::from(vec![1, 2, 3], |x| *x);
    assert_eq!(Some(0), result.find_any_index_equal_to(1));
    assert_eq!(Some(1), result.find_any_index_equal_to(2));
    assert_eq!(Some(2), result.find_any_index_equal_to(3));
    assert_eq!(None, result.find_any_index_equal_to(4));
}
