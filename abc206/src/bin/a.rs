use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    println!(
        "{}",
        if n < 191 {
            "Yay!"
        } else if n == 191 {
            "so-so"
        } else {
            ":("
        }
    );
}
