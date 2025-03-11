use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }

    b.sort_unstable();
    b.reverse();
    w.sort_unstable();
    w.reverse();
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..n {
        tmp += b[i];
        if i < m && w[i] > 0 {
            tmp += w[i];
        }
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
