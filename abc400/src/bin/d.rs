use itertools::*;
use memoise::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
    }

    let mut que = std::collections::VecDeque::new();
    let mut dist = vec![vec![!0usize; w]; h];
    que.push_back((a, b));
    dist[a][b] = 0;

    while let Some((x, y)) = que.pop_front() {
        let d = dist[x][y];
        for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < h && ny < w {
                if s[nx][ny] == b'.' {
                    if d < dist[nx][ny] {
                        dist[nx][ny] = d;
                        que.push_front((nx, ny));
                    }
                } else {
                    let d = d + 1;
                    if dist[nx][ny] > d {
                        dist[nx][ny] = d;
                        que.push_back((nx, ny));
                    }
                    let nnx = nx.wrapping_add(dx);
                    let nny = ny.wrapping_add(dy);
                    if nnx < h && nny < w && dist[nnx][nny] > d {
                        dist[nnx][nny] = d;
                        que.push_back((nnx, nny));
                    }
                }
            }
        }
    }

    println!("{}", dist[c][d]);
}
