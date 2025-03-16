use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(Usize1, Usize1); n * k - 1],
    }

    let mut e = vec![vec![]; n * k];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let x = f(0, 0, &e, k);
    if x < !0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn f(p: usize, i: usize, e: &Vec<Vec<usize>>, k: usize) -> usize {
    let mut child = vec![];
    for &j in &e[i] {
        if j == p {
            continue;
        }
        let x = f(i, j, e, k);
        if x == !0 {
            return !0;
        }
        if x > 0 {
            child.push(x);
        }
    }
    if child.len() == 0 {
        1 % k
    } else if child.len() == 1 {
        let ans = 1 + child[0];
        ans % k
    } else if child.len() == 2 {
        let ans = 1 + child[0] + child[1];
        if ans % k == 0 {
            0
        } else {
            !0
        }
    } else {
        !0
    }
}
