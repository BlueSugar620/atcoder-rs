use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    a.dedup();
    println!("{}", if a.len() == n { "Yes" } else { "No" });
}
