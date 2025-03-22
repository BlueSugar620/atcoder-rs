use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [f64; n],
    }

    let mut e = vec![0.; n + 1];
    e[0] = 1.;
    for p in &p {
        let p = p / 100.;
        let mut f = vec![0.; n + 1];
        for i in 0..n {
            f[i] += e[i] * (1. - p);
            f[i + 1] += e[i] * p;
        }
        e = f;
    }

    let mut g = vec![0.];
    for t in 1..=x {
        let mut res = 0.;
        for i in 1..=n {
            if i > t {
                break;
            }
            res += g[t - i] * e[i];
        }
        g.push((res + 1.) / (1. - e[0]));
    }

    println!("{}", g[x]);
}
