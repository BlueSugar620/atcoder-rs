use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        c: u64,
    }
    if c & 1 == 0 {
        a = a.abs();
        b = b.abs();
    }
    println!(
        "{}",
        if a > b {
            ">"
        } else if a < b {
            "<"
        } else {
            "="
        }
    );
}
