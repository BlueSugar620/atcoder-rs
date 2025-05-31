use proconio::{input, marker::Bytes};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            _n: usize,
            s: Bytes,
        }
        let mut dp0_before = 0usize;
        let mut dp1 = 0usize;
        let mut dp0_after = 0usize;
        for &c in &s {
            if c == b'0' {
                (dp0_before, dp1, dp0_after) =
                    (dp0_before, (dp0_before).min(dp1) + 1, dp1.min(dp0_after));
            } else {
                (dp0_before, dp1, dp0_after) = (
                    dp0_before + 1,
                    (dp0_before).min(dp1),
                    dp1.min(dp0_after) + 1,
                );
            }
        }
        println!("{}", dp0_before.min(dp1).min(dp0_after));
    }
}
