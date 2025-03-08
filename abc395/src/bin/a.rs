use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u16; n],
    }

    println!(
        "{}",
        if a.windows(2).all(|v| v[0] < v[1]) {
            "Yes"
        } else {
            "No"
        }
    );
}
