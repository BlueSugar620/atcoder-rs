use proconio::{input_interactive, marker::Usize1};

fn main() {
    input_interactive! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let mut dfs = vec![2; n];
    let mut stack = vec![0];
    dfs[0] = 0;
    while let Some(u) = stack.pop() {
        for &v in &e[u] {
            if dfs[v] == 2 {
                dfs[v] = dfs[u] ^ 1;
                stack.push(v);
            }
        }
    }
    let mut edges = std::collections::BTreeSet::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            if dfs[i] != dfs[j] {
                edges.insert((i, j));
            }
        }
    }
    for x in &uv {
        edges.remove(x);
    }

    if edges.len() & 1 == 1 {
        println!("First");
        let (x, y) = edges.pop_last().unwrap();
        println!("{} {}", x + 1, y + 1);
    } else {
        println!("Second");
    }

    loop {
        input_interactive! {
            i: isize,
            j: isize,
        }

        if i == -1 && j == -1 {
            return;
        }
        let i = i as usize - 1;
        let j = j as usize - 1;
        edges.remove(&(i, j));
        let (x, y) = edges.pop_last().unwrap();
        println!("{} {}", x + 1, y + 1);
    }
}
