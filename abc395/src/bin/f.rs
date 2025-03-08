use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        ud: [(u64, u64); n],
    }

    let mut l = 0;
    let mut r = 1 << 31;
    while r - l > 1 {
        let o = (l + r) / 2;
        if f(o, &x, &ud) {
            l = o;
        } else {
            r = o;
        }
    }

    println!(
        "{}",
        ud.iter().map(|x| x.0 + x.1).sum::<u64>() - l * n as u64
    );
}

fn f(h: u64, x: &u64, ud: &[(u64, u64)]) -> bool {
    let mut l = 0;
    let mut r = h;
    for &(u, d) in ud {
        l = (l + d).max(h + x).max(x + d) - *x - d;
        r = (r + x).min(u);
        if l > r {
            return false;
        }
    }
    true
}
