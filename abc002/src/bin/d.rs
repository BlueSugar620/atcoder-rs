use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut e = vec![vec![false; n]; n];
    for &(x, y) in &xy {
        e[x][y] = true;
        e[y][x] = true;
    }

    let ans = (0usize..1 << n)
        .map(|b| {
            if (0..n).all(|i| {
                (0..n).all(|j| i == j || (b >> i) & 1 == 0 || (b >> j) & 1 == 0 || e[i][j])
            }) {
                b.count_ones()
            } else {
                0
            }
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
