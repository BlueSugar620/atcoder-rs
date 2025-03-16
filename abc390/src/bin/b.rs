use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    if a.windows(3).all(|v| v[0] * v[2] == v[1] * v[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
