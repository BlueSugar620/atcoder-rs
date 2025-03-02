use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [u64; n],
    }

    r.sort_unstable();
    r.reverse();

    let ans = r[..k]
        .iter()
        .enumerate()
        .map(|(i, r)| *r as f64 / (1u128 << (i + 1)) as f64)
        .sum::<f64>();

    println!("{}", ans);
}
