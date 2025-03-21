use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: u64,
        a: [u64; n],
    }

    s %= a.iter().sum::<u64>();
    let mut r = 0;
    let mut t = 0;
    for &ai in a.iter().chain(a.iter()) {
        while r < 2 * n && t < s {
            t += a[r % n];
            r += 1;
        }
        if t == s {
            println!("Yes");
            return;
        }
        t -= ai;
    }
    println!("No");
}
