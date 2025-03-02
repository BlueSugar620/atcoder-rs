use proconio::input;

fn main() {
    input! {
        r: i64,
        g: i64,
        b: i64,
    }

    let ans = (-300..300)
        .map(|lg| {
            let lr = (lg - r).min(-100 - r / 2);
            let lb = (lg + g).max(100 - b / 2);
            cnt(lr, lr + r, -100) + cnt(lg, lg + g, 0) + cnt(lb, lb + b, 100)
        })
        .min()
        .unwrap();

    println!("{}", ans);
}

fn cnt(l: i64, r: i64, c: i64) -> i64 {
    if c < l {
        return (r - c - 1) * (r - c) / 2 - (l - c - 1) * (l - c) / 2;
    } else if c < r {
        return (r - c - 1) * (r - c) / 2 + (c - l) * (c - l + 1) / 2;
    } else {
        return (c - l) * (c - l + 1) / 2 - (c - r) * (c - r + 1) / 2;
    }
}
