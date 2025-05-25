use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    println!("{}", k * n * (n + 1) * 50 + n * k * (k + 1) / 2);
}
