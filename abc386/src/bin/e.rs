use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }

    println!(
        "{}",
        if k <= n / 2 {
            a.iter()
                .combinations(k)
                .map(|a| a.iter().fold(0, |acc, a| acc ^ *a))
                .max()
                .unwrap()
        } else {
            let x = a.iter().fold(0, |acc, a| acc ^ a);
            a.iter()
                .combinations(n - k)
                .map(|a| x ^ a.iter().fold(0, |acc, a| acc ^ *a))
                .max()
                .unwrap()
        }
    );
}
