use proconio::input;

fn main() {
    input! {
        mut a: [u8; 3],
    }
    a.sort_unstable();
    println!("{}", if a[0] * a[1] == a[2] { "Yes" } else { "No" });
}
