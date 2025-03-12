use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        ask: [(Usize1, usize); q],
    }
    let ord = (0..q)
        .sorted_unstable_by_key(|i| ask[*i].0)
        .collect::<Vec<_>>();
    let mut idx = 0;
    let mut ans = vec![!0; q];
    let n = a.len();
    let mut dp = vec![!0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let o = (l + r) / 2;
            if dp[o] < a {
                l = o;
            } else {
                r = o;
            }
        }
        dp[r] = a;

        while idx < q && ask[ord[idx]].0 == i {
            ans[ord[idx]] = dp.partition_point(|x| *x <= ask[ord[idx]].1) - 1;
            idx += 1;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
