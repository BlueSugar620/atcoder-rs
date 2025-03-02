use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: [[char; 4]; 4],
    }

    println!(
        "{}",
        c.iter().rev().map(|c| c.iter().rev().join(" ")).join("\n")
    );
}
