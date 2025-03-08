use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut his = [!0; 1_000_000 + 1];
    let mut ans = !0;
    for (i, &a) in a.iter().enumerate() {
        if his[a] == !0 {
            his[a] = i;
        } else {
            ans = ans.min(i - his[a] + 1);
            his[a] = i;
        }
    }

    println!("{}", ans as isize);
}
