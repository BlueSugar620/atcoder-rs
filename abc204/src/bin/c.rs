use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut stack = vec![i];
        let mut flag = vec![false; n];
        flag[i] = true;
        while let Some(u) = stack.pop() {
            for &v in &e[u] {
                if !flag[v] {
                    flag[v] = true;
                    stack.push(v);
                }
            }
        }
        ans += flag.iter().filter(|t| **t).count();
    }
    println!("{}", ans);
}
