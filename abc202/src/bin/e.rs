use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        q: usize,
        ask: [(Usize1, usize); q],
    }
    let mut e = vec![vec![]; n];
    for (i, p) in p.iter().enumerate() {
        e[*p].push(i + 1);
    }
    let mut in_time = vec![0; n];
    let mut out_time = vec![0; n];
    let mut depth = vec![vec![]; n];
    dfs(
        0,
        &mut 0,
        &mut 0,
        &mut in_time,
        &mut out_time,
        &mut depth,
        &e,
    );

    for &(u, d) in &ask {
        let ans = depth[d].upper_bound(&out_time[u]) - depth[d].lower_bound(&in_time[u]);
        println!("{}", ans);
    }
}

fn dfs(
    i: usize,
    time: &mut usize,
    d: &mut usize,
    in_time: &mut [usize],
    out_time: &mut [usize],
    depth: &mut Vec<Vec<usize>>,
    e: &Vec<Vec<usize>>,
) {
    in_time[i] = *time;
    depth[*d].push(*time);
    for &j in &e[i] {
        *time += 1;
        *d += 1;
        dfs(j, time, d, in_time, out_time, depth, e);
        *d -= 1;
    }
    out_time[i] = *time;
}
