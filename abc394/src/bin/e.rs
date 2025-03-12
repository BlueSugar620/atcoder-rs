use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        c: [Bytes; n],
    }

    let mut dist = vec![vec![!0u64; n]; n];
    let mut que = std::collections::VecDeque::new();

    for i in 0..n {
        dist[i][i] = 0;
        que.push_back((i, i));
    }

    for i in 0..n {
        for j in 0..n {
            if i != j && c[i][j] != b'-' {
                dist[i][j] = 1;
                que.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = que.pop_front() {
        for x in 0..n {
            for y in 0..n {
                if c[x][i] != b'-' && c[x][i] == c[j][y] && dist[i][j] + 2 < dist[x][y] {
                    dist[x][y] = dist[i][j] + 2;
                    que.push_back((x, y));
                }
            }
        }
    }

    println!(
        "{}",
        dist.iter()
            .map(|d| d.iter().map(|d| *d as i64).join(" "))
            .join("\n")
    );
}
