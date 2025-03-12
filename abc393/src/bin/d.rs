use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    }

    let x = s.iter().filter(|c| **c == b'1').count();
    let mut ans = 0;
    let mut cnt = 0;
    for &c in &s {
        if c == b'0' {
            ans += cnt.min(x - cnt);
        } else {
            cnt += 1;
        }
    }
    println!("{}", ans);
}
