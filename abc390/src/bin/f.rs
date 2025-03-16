use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut f = vec![vec![0]; n + 1];
    let mut g = vec![vec![0]; n + 1];
    for (i, &a) in a.iter().enumerate() {
        f[a].push(i + 1);
        g[a].push(i + 1);
        if a > 0 {
            g[a - 1].push(i + 1);
        }
    }
    for f in f.iter_mut() {
        f.push(n + 1);
    }
    for g in g.iter_mut() {
        g.push(n + 1);
    }

    let ans = f
        .iter()
        .map(|f| {
            f.windows(2)
                .map(|v| (v[1] - v[0]) * (v[1] - v[0] - 1) / 2)
                .sum::<usize>()
        })
        .sum::<usize>()
        - g.iter()
            .map(|g| {
                g.windows(2)
                    .map(|v| (v[1] - v[0]) * (v[1] - v[0] - 1) / 2)
                    .sum::<usize>()
            })
            .sum::<usize>();

    println!("{}", ans);
}
