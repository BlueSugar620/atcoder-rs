use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    let mut prev = 0;
    for i in (0..30).rev() {
        let mut now = 0;
        let mut map = std::collections::HashMap::new();
        for &a in &a {
            let p = ((1 << i) - a % (1 << i)) % (1 << i);
            let (c, t) = map.get(&p).unwrap_or(&(0, 0));
            map.insert(p, (c + 1, t + a));
            let (c, t) = map.get(&(a % (1 << i))).unwrap_or(&(0, 0));
            now += a * c + t;
        }
        ans += (now - prev) >> i;
        prev = now;
    }

    println!("{}", ans);
}
