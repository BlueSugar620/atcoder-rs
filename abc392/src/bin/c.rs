use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }
    let mut r = vec![0; n];
    for (i, q) in q.iter().enumerate() {
        r[*q] = i;
    }

    println!("{}", r.iter().map(|r| q[p[*r]] + 1).join(" "));
}
