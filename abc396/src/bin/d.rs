use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, u64); m],
    }

    let mut e = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        e[u].push((v, w));
        e[v].push((u, w));
    }

    let mut used = vec![false; n];
    let mut ans = !0;
    dfs(0, &mut used, 0, &e, &mut ans);

    println!("{}", ans);
}

fn dfs(i: usize, used: &mut Vec<bool>, xor: u64, e: &Vec<Vec<(usize, u64)>>, ans: &mut u64) {
    if i == e.len() - 1 {
        if xor < *ans {
            *ans = xor;
        }
        return;
    }

    used[i] = true;
    for &(j, w) in &e[i] {
        if !used[j] {
            dfs(j, used, xor ^ w, e, ans);
        }
    }
    used[i] = false;
}
