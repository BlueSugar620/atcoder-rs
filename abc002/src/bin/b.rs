use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        w: Chars,
    }

    println!(
        "{}",
        w.iter()
            .filter(|c| !['a', 'i', 'u', 'e', 'o'].contains(*c))
            .join("")
    );
}
