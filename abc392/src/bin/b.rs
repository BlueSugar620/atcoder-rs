use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    println!("{}", n - m);
    println!("{}", (1..=n).filter(|i| !a.contains(i)).join(" "));
}
