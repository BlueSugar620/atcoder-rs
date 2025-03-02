use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut now = [1, 2, 3, 4, 5, 6];
    for i in 0..n % 30 {
        now.swap(i % 5, i % 5 + 1);
        if now == [1, 2, 3, 4, 5, 6] {
            break;
        }
    }

    println!("{}", now.iter().join(""));
}
