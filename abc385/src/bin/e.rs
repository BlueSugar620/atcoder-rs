use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let ans = (0..n)
        .map(|i| {
            let mut ans = 0;
            let mut deg = e[i].iter().map(|j| e[*j].len()).collect::<Vec<_>>();
            deg.sort_unstable();
            deg.reverse();
            for (i, d) in deg.iter().enumerate() {
                if *d < 2 {
                    break;
                }
                ans = ans.max(1 + (i + 1) * d);
            }
            ans
        })
        .max()
        .unwrap();
    println!("{}", n - ans);
}
