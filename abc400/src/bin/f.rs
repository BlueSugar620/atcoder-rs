use itertools::*;
use memoise::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        x: [usize; n],
    }

    let mut dp = vec![vec![!0 / 2; 2 * n + 1]; 2 * n + 1];
    for i in 0..2 * n {
        dp[i][i + 1] = x[c[i % n]] + 1;
    }

    let mut ep = vec![vec![!0 / 2; 2 * n + 1]; 2 * n + 1];
    for i in 0..2 * n {
        ep[i][i + 1] = 0;
    }

    for d in 2..=n {
        for l in 0..2 * n - d {
            let r = l + d;
            for o in l + 1..r {
                dp[l][r] = dp[l][r].min(dp[l][o] + dp[o][r]);
            }
            for o in l + 1..r {
                ep[l][r] = ep[l][r].min(ep[l][o] + dp[o][r]);
            }
            if c[l % n] == c[(r - 1) % n] {
                ep[l][r] = ep[l][r].min(ep[l][r - 1]);
            };
            if c[l % n] == c[(r - 1) % n] {
                dp[l][r] = dp[l][r].min(ep[l][r] + r - l + x[c[l % n]]);
            }
        }
    }

    let mut ans = !0;
    for i in 0..n {
        ans = ans.min(dp[i][i + n]);
    }
    println!("{}", ans);
}
