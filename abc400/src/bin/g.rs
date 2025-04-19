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
const INF: i64 = 1 << 50;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            mut xyz: [[i64; 3]; n],
        }

        xyz.sort_unstable_by_key(|xyz| *xyz.iter().max().unwrap());
        xyz.reverse();

        // dp[i][t][b]: [0, i) のなかでスルーしたのが t 個で現在の選択の偶奇状況が b
        // であるようなものの最大値
        let mut dp = [[-INF; 8]; 3];
        dp[0][0] = 0;

        for xyz in xyz[..2 * k].iter() {
            let mut ep = [[-INF; 8]; 3];
            // k として選択する。
            for k in 0..3 {
                for t in 0..=2 {
                    for b in 0..8 {
                        ep[t][b ^ (1 << k)] = ep[t][b ^ (1 << k)].max(dp[t][b] + xyz[k]);
                    }
                }
            }
            // 何も選択せずにスルーする。
            for t in 0..2 {
                for b in 0..8 {
                    ep[t + 1][b] = ep[t + 1][b].max(dp[t][b]);
                }
            }

            dp = ep;
        }

        for xyz in xyz[2 * k..].iter() {
            let mut ep = [[-INF; 8]; 3];
            // 何も選択せずにスルーする。
            for t in 0..=2 {
                for b in 0..8 {
                    ep[t][b] = ep[t][b].max(dp[t][b]);
                }
            }
            // k として選択する。
            for k in 0..3 {
                for t in 1..=2 {
                    for b in 0..8 {
                        ep[t - 1][b ^ (1 << k)] = ep[t - 1][b ^ (1 << k)].max(dp[t][b] + xyz[k]);
                    }
                }
            }

            dp = ep;
        }

        println!("{}", dp[0][0]);
    }
}
