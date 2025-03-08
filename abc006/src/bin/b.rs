use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = 0usize;
    let mut b = 0;
    let mut c = 1;

    for _ in 0..n - 1 {
        (a, b, c) = (b, c, (a + b + c) % 10_007);
    }

    println!("{}", a);
}
