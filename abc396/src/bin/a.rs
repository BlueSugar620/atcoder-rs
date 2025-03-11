use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }

    println!(
        "{}",
        if a.windows(3).any(|a| a[0] == a[1] && a[1] == a[2]) {
            "Yes"
        } else {
            "No"
        }
    );
}
