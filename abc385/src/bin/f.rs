use proconio::input;

fn main() {
    input! {
        n: usize,
        xh: [(i64, i64); n],
    }

    println!(
        "{}",
        if xh.windows(2).all(|v| {
            let (x0, h0) = v[0];
            let (x1, h1) = v[1];
            h0 * x1 - h1 * x0 < 0
        }) {
            -1.0
        } else {
            xh.windows(2)
                .map(|v| {
                    let (x0, h0) = v[0];
                    let (x1, h1) = v[1];
                    (h0 * x1 - h1 * x0) as f64 / (x1 - x0) as f64
                })
                .fold(0f64, |acc, a| acc.max(a))
        }
    );
}
