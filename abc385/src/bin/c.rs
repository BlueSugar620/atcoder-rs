use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for d in 1..=n {
            let height = h[i];
            let mut tmp = 1;
            while i + d * tmp < n && h[i + d * tmp] == height {
                tmp += 1;
            }
            ans = ans.max(tmp);
        }
    }

    println!("{}", ans);
}
