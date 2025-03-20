use proconio::input;

fn main() {
    input! {
        mut a: [u16; 3],
    }
    a.sort_unstable();
    println!(
        "{}",
        if a[0] + a[1] == a[2] || (a[0] == a[1] && a[1] == a[2]) {
            "Yes"
        } else {
            "No"
        }
    );
}
