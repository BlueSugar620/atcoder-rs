use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    println!(
        "{}",
        s.iter()
            .sorted_by_key(|s| s.len())
            .map(|s| s.iter().join(""))
            .join("")
    );
}
