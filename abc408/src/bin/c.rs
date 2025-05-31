use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, usize); m],
    }
    let mut freq = vec![0i32; n + 1];
    for &(l, r) in &lr {
        freq[l] += 1;
        freq[r] -= 1;
    }
    let mut ans = std::i32::MAX;
    for i in 0..n {
        if i > 0 {
            freq[i] += freq[i - 1];
        }
        ans = ans.min(freq[i]);
    }
    println!("{}", ans);
}
