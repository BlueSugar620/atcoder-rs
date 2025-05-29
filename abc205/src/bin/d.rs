use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        k: [usize; q],
    }
    let mut ans = vec![!0; q];
    let idx = (0..q).sorted_unstable_by_key(|i| k[*i]).collect::<Vec<_>>();
    let mut now = 0;
    for &idx in &idx {
        let k = k[idx];
        let mut x = k + now;
        while now < n && a[now] <= x {
            x += 1;
            now += 1;
        }
        ans[idx] = x;
    }
    println!("{}", ans.iter().join("\n"));
}
