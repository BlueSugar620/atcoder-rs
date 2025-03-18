use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    }

    let start = iproduct!(0..h, 0..w)
        .find(|&(i, j)| s[i][j] == b'S')
        .unwrap();
    let goal = iproduct!(0..h, 0..w)
        .find(|&(i, j)| s[i][j] == b'G')
        .unwrap();

    let mut d = [vec![vec![!0usize; w]; h], vec![vec![!0usize; w]; h]];
    let mut que = std::collections::VecDeque::new();

    d[0][start.0][start.1] = 0;
    d[1][start.0][start.1] = 0;
    que.push_back((start, 0));
    que.push_back((start, 1));

    while let Some(((x, y), t)) = que.pop_front() {
        let p = d[t][x][y];
        for (dx, dy) in if t == 0 {
            [(1, 0), (!0, 0)]
        } else {
            [(0, 1), (0, !0)]
        } {
            let x = x.wrapping_add(dx);
            let y = y.wrapping_add(dy);
            if x < h && y < w && s[x][y] != b'#' && p + 1 < d[t ^ 1][x][y] {
                d[t ^ 1][x][y] = p + 1;
                que.push_back(((x, y), t ^ 1));
            }
        }
    }

    println!(
        "{}",
        d[0][goal.0][goal.1].min(d[1][goal.0][goal.1]) as isize
    );
}
