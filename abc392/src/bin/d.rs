use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut d = vec![];
    let mut len = vec![];
    for _ in 0..n {
        input! {
            k: usize,
            a: [usize; k],
        }
        let mut cnt = vec![0usize; 100_000 + 1];
        for &a in &a {
            cnt[a] += 1;
        }
        len.push(k as f64);
        d.push(cnt);
    }

    let mut ans = 0.;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let tot = d[i]
                .iter()
                .zip(d[j].iter())
                .map(|x| x.0 * x.1)
                .sum::<usize>();
            let x = tot as f64 / len[i] / len[j];
            if x > ans {
                ans = x;
            }
        }
    }

    println!("{}", ans);
}
