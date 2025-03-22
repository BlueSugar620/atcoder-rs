use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: usize,
        k: u64,
        mut puc: [(usize, u64, Usize1); n],
    }

    let mut idx = 0;
    puc.sort_unstable_by_key(|a| a.2);
    let mut dp = vec![0; x + 1];
    for i in 0..n {
        let mut ep = vec![0; x + 1];
        while idx < n && puc[idx].2 == i {
            let (p, u, _) = puc[idx];
            for j in (p..=x).rev() {
                ep[j] = (dp[j - p] + u + k).max(ep[j - p] + u).max(ep[j]);
            }
            idx += 1;
        }
        for j in 0..=x {
            ep[j] = dp[j].max(ep[j]);
        }
        dp = ep;
    }

    println!("{}", dp.iter().max().unwrap());
}
