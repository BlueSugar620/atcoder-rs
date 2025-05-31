use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        mut t: [u32; n],
    }
    t.insert(0, 0);
    println!(
        "{}",
        if t.windows(2).all(|t| t[1] - t[0] <= s) {
            "Yes"
        } else {
            "No"
        }
    );
}
