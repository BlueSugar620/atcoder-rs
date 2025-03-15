use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Bytes; n],
        t: [Bytes; m],
    }

    for a in 0..=n - m {
        for b in 0..=n - m {
            if iproduct!(0..m, 0..m).all(|(i, j)| s[a + i][b + j] == t[i][j]) {
                println!("{} {}", a + 1, b + 1);
            }
        }
    }
}
