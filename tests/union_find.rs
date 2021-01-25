extern crate leetcode_utils;
use leetcode_utils::union_find::*;

#[test]
fn test1() {
    let mut uf = UnionFind4Usize::new(4);
    uf.union(1, 2);
    uf.union(2, 3);
    assert_eq!(2, uf.union_count());
    assert_eq!(1, uf.find(3));
    assert_eq!(1, uf.find(2));
    assert_eq!(1, uf.find(1));
    assert_eq!(0, uf.find(0));
    assert_eq!(3, uf.union_size(3));
    assert_eq!(4, uf.len());
}

#[test]
fn test2() {
    let mut uf = UnionFind4Usize::new(0);
    assert_eq!(0, uf.union_count());

    assert_eq!(0, uf.add());
    assert_eq!(1, uf.add());
    uf.union(0, 1);

    assert_eq!(1, uf.union_count());
    assert_eq!(0, uf.find(0));
    assert_eq!(0, uf.find(1));
    assert_eq!(2, uf.union_size(1));
    assert_eq!(2, uf.len());
}

#[test]
fn test3() {
    let mut uf = UnionFind::new();
    uf.add(100);
    uf.add(200);
    uf.add(300);
    assert!(uf.union(100, 200).is_ok());
    assert!(uf.union(100, 400).is_err());
    assert_eq!(Some(&100), uf.find(200));
    assert_eq!(2, uf.union_count());
    assert_eq!(Some(2), uf.union_size(100));
}

#[test]
fn test4() {
    let list = vec!["a", "b", "c"];
    let mut uf = UnionFind::from_iter(list);
    assert!(uf.union("a", "b").is_ok());
    assert!(uf.union("c", "b").is_ok());
    assert_eq!(Some(&"a"), uf.find("a"));
    assert_eq!(Some(&"a"), uf.find("b"));
    assert_eq!(Some(&"a"), uf.find("c"));
    assert_eq!(1, uf.union_count());
    assert_eq!(Some(3), uf.union_size("a"));
    assert_eq!(Some(3), uf.union_size("b"));
    assert_eq!(Some(3), uf.union_size("c"));
}

#[test]
fn test5() {
    let list = vec![(0, 0, 0), (0, 1, 1), (2, 1, 1)];
    let mut uf = UnionFind::from_iter(list);
    assert!(uf.union((0, 0, 0), (0, 1, 1)).is_ok());
}
