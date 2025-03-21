use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        c1: char,
        c2: char,
        s: String,
    }

    println!(
        "{}",
        s.chars().map(|c| if c != c1 { c2 } else { c }).join("")
    );
}
