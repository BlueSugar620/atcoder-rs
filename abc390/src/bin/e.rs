use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        x: usize,
        vac: [(Usize1, usize, usize); n],
    }

    // dp[i][y]: yカロリー以下で取れるビタミンiの最大量
    let mut dp = vec![vec![0; x + 1]; 3];
    for &(v, a, c) in &vac {
        let mut swp = dp.clone();
        for y in 0..=x {
            if y + c > x {
                break;
            }
            swp[v][y + c] = swp[v][y + c].max(dp[v][y] + a);
        }
        dp = swp;
    }

    for i in 0..3 {
        for y in 0..x {
            dp[i][y + 1] = dp[i][y + 1].max(dp[i][y]);
        }
    }

    let mut ok = 0;
    let mut ng = 1 << 30;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let a = dp.iter().map(|dp| dp.lower_bound(&mid)).sum::<usize>();
        if a <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
