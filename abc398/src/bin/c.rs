use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut b = std::collections::HashMap::new();
    for &a in &a {
        *b.entry(a).or_insert(0) += 1;
    }

    let mut ans = !0;
    let mut cnt = 0;
    for (i, &a) in a.iter().enumerate() {
        if *b.get(&a).unwrap() == 1 {
            if a > cnt {
                ans = i + 1;
                cnt = a;
            }
        }
    }

    println!("{}", ans as isize);
}
