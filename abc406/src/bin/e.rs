use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use rand::{thread_rng, Rng};
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;
type Heap<T> = BinaryHeap<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        t: usize,
        ask: [(usize, usize); t],
    }

    for &(n, k) in &ask {
        // dp[x][t]: bitの数が x 個で、 t: n 以下が確定しているかどうか
        // (これまでの総和, 何個あったか)
        let mut dp = [[(0, 0), (0, 0)]; 64];
        dp[0][0] = (0, 1);
        for i in (0..64).rev() {
            let bit = (n >> i) & 1;
            let mut ep = [[(0, 0), (0, 0)]; 64];
            for x in 0..63 {
                // 確定しているものに 0 を加える
                ep[x][1] = (
                    (ep[x][1].0 + dp[x][1].0 * 2) % MOD,
                    (ep[x][1].1 + dp[x][1].1) % MOD,
                );
                // 確定しているものに 1 を加える
                ep[x + 1][1] = (
                    (ep[x + 1][1].0 + dp[x][1].0 * 2 + dp[x][1].1) % MOD,
                    (ep[x + 1][1].1 + dp[x][1].1) % MOD,
                );
                if bit == 0 {
                    // 0 を確定していないものにつけると確定しない
                    ep[x][0] = (dp[x][0].0 * 2 % MOD, dp[x][0].1 % MOD);
                } else {
                    // 0 を確定していないものにつけると確定する
                    ep[x][1] = (
                        (ep[x][1].0 + 2 * dp[x][0].0) % MOD,
                        (ep[x][1].1 + dp[x][0].1) % MOD,
                    );
                    // 1 を確定していないものにつけると確定しない
                    ep[x + 1][0] = ((dp[x][0].0 * 2 + dp[x][0].1) % MOD, dp[x][0].1 % MOD);
                }
            }
            dp = ep;
        }
        println!("{}", (dp[k][0].0 + dp[k][1].0) % MOD);
    }
}
