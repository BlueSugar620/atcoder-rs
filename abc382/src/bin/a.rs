use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: Bytes,
    }

    println!("{}", n.min(s.iter().filter(|c| **c == b'.').count() + d));
}
