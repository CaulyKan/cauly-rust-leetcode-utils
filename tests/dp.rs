extern crate cauly_rust_leetcode_utils;
use cauly_rust_leetcode_utils::define_dp;
use cauly_rust_leetcode_utils::dp::*;

struct MyDp {
    coins: usize,
}
struct MyDp2 {
    coins: usize,
    round: usize,
}
struct MyDp3 {
    coins: usize,
    round: usize,
    owner: usize,
}

define_dp! {MyDp, coins:10}
define_dp! {MyDp2, coins:10, round:10}
define_dp! {MyDp3, coins:10, round:10, owner:10}

#[test]
fn test1() {
    let dp = DP::new::<MyDp>(0);
    assert_eq!(0, dp.get(&MyDp { coins: 0 }));
}

#[test]
fn test2() {
    let mut dp = DP::new::<MyDp2>(-1);
    assert_eq!(-1, dp.get(&MyDp2 { coins: 0, round: 0 }));
    *dp.get_mut(&MyDp2 { coins: 0, round: 0 }) = dp.get(&MyDp2 { coins: 0, round: 0 }) + 2;
    assert_eq!(1, dp.get(&MyDp2 { coins: 0, round: 0 }));
    *dp.get_mut(&MyDp2 { coins: 0, round: 0 }) = dp.get(&MyDp2 { coins: 0, round: 0 }) + 2;
    assert_eq!(3, dp.get(&MyDp2 { coins: 0, round: 0 }));
}

#[test]
fn test3() {
    let mut dp = DP::new::<MyDp3>(0);
    let mut current = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                *dp.get_mut(&MyDp3 {
                    coins: i,
                    round: j,
                    owner: k,
                }) = current;
                current += 1;
            }
        }
    }

    current = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                assert_eq![
                    current,
                    dp.get(&MyDp3 {
                        coins: i,
                        round: j,
                        owner: k,
                    })
                ];
                current += 1;
            }
        }
    }
}
