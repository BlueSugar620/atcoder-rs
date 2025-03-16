use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    let mut diff = 0;
    let mut idx = 0;
    for (i, &ai) in a.iter().enumerate() {
        while idx < n && a[idx] < 2 * ai {
            idx += 1;
        }
        diff = diff.max(idx - i);
        if i + diff.max(i) < n {
            ans = i + 1;
        }
    }

    println!("{}", ans);
}
