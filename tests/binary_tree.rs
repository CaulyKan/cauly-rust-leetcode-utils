extern crate leetcode_utils;
use leetcode_utils::binary_tree::*;

#[test]
fn test1() {
    let result = TreeNode::from_string("1").unwrap();
    assert_eq!(1, result.borrow().val);
}

#[test]
fn test2() {
    let result = TreeNode::from_string("1,2,3");
    result.walk(TraversalType::Inorder, |mut x| {
        x.set_val(x.get_val().unwrap() + 1)
    });
    assert_eq!(Some(2), result.get_val());
    assert_eq!(Some(3), result.get_left().get_val());
    assert_eq!(Some(4), result.get_right().get_val());
}
#[test]
fn test3() {
    let result = TreeNode::from_string("1,null,3,4,5");
    assert_eq!(Some(1), result.get_val());
    assert_eq!(None, result.get_left());
    assert_eq!(Some(3), result.get_right().get_val());
    assert_eq!(Some(4), result.get_right().get_left().get_val());
    assert_eq!(Some(5), result.get_right().get_right().get_val());
}

#[test]
fn test4() {
    let result = TreeNode::from_string("1,null,null");
    assert_eq!(Some(1), result.get_val());
    assert_eq!(None, result.get_left());
    assert_eq!(None, result.get_right());
}

#[test]
fn test5() {
    let result = TreeNode::from_string("1,2,3,4,5");
    let sum = result.aggregate(
        |n| n.get_val().unwrap_or(0),
        |v, l, r| Some(v + l.unwrap_or(0) + r.unwrap_or(0)),
    );
    assert_eq!(Some(1 + 2 + 3 + 4 + 5), sum);
}

#[test]
fn test6() {
    let result = TreeNode::from_string("1,2,3");
    let mut v = Vec::new();
    result.walk_t(TraversalType::Inorder, &mut v, |v, x| match x.get_val() {
        Some(i) => v.push(i),
        _ => (),
    });
    assert_eq!(vec![2, 1, 3], v);
}

#[test]
fn test7() {
    let result = TreeNode::from_string("1,null,2,3");
    assert_eq!(Some(3), result.get_right().get_left().get_val());
}

#[test]
fn test8() {
    let result = TreeNode::from_string("1,null,2,3");
    let mut count = 0;
    let sum = result.aggregate_t(
        &mut count,
        |_, node| node.get_val().unwrap_or(0),
        |count, val, lval, rval| {
            *count += 1;
            Some(val + lval.unwrap_or(0) + rval.unwrap_or(0))
        },
    );
    assert_eq!(3, count);
    assert_eq!(Some(6), sum);
}
