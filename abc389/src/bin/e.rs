use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u128,
        p: [u128; n],
    }

    let mut l = 0;
    let mut r = 1 << 60;
    while r - l > 1 {
        let o = (l + r) / 2;
        let tot = p
            .iter()
            .map(|p| ((o + p) / (2 * p)).pow(2) * p)
            .sum::<u128>();
        if tot <= m {
            l = o;
        } else {
            r = o;
        }
    }

    let mut tot = 0;
    let mut tmp = 0;
    let mut ans = 0;
    for &p in &p {
        let cnt = (l + p) / (2 * p);
        ans += cnt;
        tot += cnt * cnt * p;
        if (2 * (cnt + 1) - 1) * p == r {
            tmp += 1;
        }
    }
    while tmp > 0 && tot + r <= m {
        tmp -= 1;
        ans += 1;
        tot += r;
    }

    println!("{}", ans);
}
