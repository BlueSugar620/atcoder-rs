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
        k: usize,
        s: [Bytes; h],
    }

    let mut v = vec![];
    let mut ans = 0;
    dfs(&mut v, &mut ans, h, w, k, &s);
    println!("{}", ans);
}

pub fn dfs(
    v: &mut Vec<(usize, usize)>,
    ans: &mut usize,
    h: usize,
    w: usize,
    k: usize,
    s: &Vec<Vec<u8>>,
) {
    if v.len() == k + 1 {
        *ans += 1;
        return;
    }

    if v.len() == 0 {
        for i in 0..h {
            for j in 0..w {
                if s[i][j] == b'.' {
                    v.push((i, j));
                    dfs(v, ans, h, w, k, s);
                    v.pop();
                }
            }
        }
    } else {
        let last = v.last().unwrap().clone();
        for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let x = last.0.wrapping_add(dx);
            let y = last.1.wrapping_add(dy);
            if x < h && y < w && s[x][y] == b'.' && !v.contains(&(x, y)) {
                v.push((x, y));
                dfs(v, ans, h, w, k, s);
                v.pop();
            }
        }
    }
}
