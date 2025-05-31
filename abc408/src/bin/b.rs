use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    a.sort_unstable();
    a.dedup();
    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}
