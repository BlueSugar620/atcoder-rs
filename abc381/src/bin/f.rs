use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut next = vec![];
    let mut memo = [!0; 20];
    for (i, &a) in a.iter().enumerate().rev() {
        memo[a] = i + 1;
        next.push(memo);
    }
    next.reverse();

    let mut dp = vec![!0; 1 << 20];
    dp[0] = 0;
    for b in 0..1 << 20 {
        if dp[b] >= n {
            continue;
        }
        for i in 0..20 {
            let x = next[dp[b]][i];
            if x >= n {
                continue;
            }
            let y = next[x][i];
            if y > n {
                continue;
            }
            dp[b | (1 << i)] = dp[b | (1 << i)].min(y);
        }
    }

    println!(
        "{}",
        2 * dp
            .iter()
            .enumerate()
            .map(|(bit, dp)| if *dp == !0 { 0 } else { bit.count_ones() })
            .max()
            .unwrap()
    );
}
