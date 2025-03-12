use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        f: [[u64; w]; h],
        q: usize,
        ask: [(Usize1, Usize1, u64, Usize1, Usize1, u64); q],
    }
}
