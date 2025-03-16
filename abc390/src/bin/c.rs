use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    }

    let mut x = (!0, 0);
    let mut y = (!0, 0);

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                x = (x.0.min(i), x.1.max(i));
                y = (y.0.min(j), y.1.max(j));
            }
        }
    }

    if iproduct!(0..h, 0..w).all(|(i, j)| {
        if x.0 <= i && i <= x.1 && y.0 <= j && j <= y.1 {
            s[i][j] != b'.'
        } else {
            s[i][j] != b'#'
        }
    }) {
        println!("Yes");
    } else {
        println!("No");
    }
}
