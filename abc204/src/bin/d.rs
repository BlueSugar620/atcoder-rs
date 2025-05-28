use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let m = t.iter().sum::<usize>();
    let mut dp = vec![false; m + 1];
    dp[0] = true;
    for &t in &t {
        let mut ndp = dp.clone();
        for i in t..=m {
            if dp[i - t] {
                ndp[i] = true;
            }
        }
        dp = ndp;
    }
    println!(
        "{}",
        (0..=m)
            .filter(|i| dp[*i])
            .map(|i| i.max(m - i))
            .min()
            .unwrap()
    );
}
