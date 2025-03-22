use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut d: usize,
        mut s: Chars,
    }

    for c in s.iter_mut().rev() {
        if d > 0 && *c == '@' {
            *c = '.';
            d -= 1;
        }
    }

    println!("{}", s.iter().join(""));
}
