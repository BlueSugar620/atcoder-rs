use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    for i in (0..s.len() - 1).rev() {
        if s[i] == 'W' && s[i + 1] == 'A' {
            s[i] = 'A';
            s[i + 1] = 'C';
        }
    }
    println!("{}", s.iter().join(""));
}
