#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::BinaryHeap;
#[fastout]
fn main() {
    input! {
        n: usize,
        mut m: usize,
        mut a: [usize; n],
    }
    let mut queue = BinaryHeap::new();
    for i in 0..n {
        queue.push(a[i]);
    }
    for _ in 0..m {
        let t = queue.pop().unwrap();
        queue.push(t / 2);
    }
    let mut ans = 0;
    while !queue.is_empty() {
        ans += queue.pop().unwrap();
    }
    println!("{}", ans);
}
