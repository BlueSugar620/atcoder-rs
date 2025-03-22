use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(u32, u32); n],
    }

    let mut tot = 0u32;
    let mut now = 0;
    for &(t, v) in &tv {
        tot = tot.saturating_sub(t - now);
        now = t;
        tot += v;
    }
    println!("{}", tot);
}
