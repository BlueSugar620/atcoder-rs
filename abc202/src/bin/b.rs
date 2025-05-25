use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.reverse();
    s.iter_mut().for_each(|c| {
        if *c == '6' {
            *c = '9';
        } else if *c == '9' {
            *c = '6'
        }
    });
    println!("{}", s.iter().join(""));
}
