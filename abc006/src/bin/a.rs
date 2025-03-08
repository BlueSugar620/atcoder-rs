use proconio::input;

fn main() {
    input! {
        n: u8,
    }
    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
