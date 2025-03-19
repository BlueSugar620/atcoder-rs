use proconio::input;

fn main() {
    input! {
        mut x: [u8; 4],
    }

    x.sort_unstable();
    x.dedup();
    println!("{}", if x.len() == 2 { "Yes" } else { "No" });
}
