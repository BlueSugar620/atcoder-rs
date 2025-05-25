use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_unstable();
    let mut now = 0;
    for &(a, b) in &ab {
        if now + k < a {
            now += k;
            k = 0;
            break;
        }
        k -= a - now;
        k += b;
        now = a;
    }
    now += k;
    println!("{}", now);
}
