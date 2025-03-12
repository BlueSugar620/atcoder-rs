use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    println!(
        "{}",
        if s1 == "fine" { 2 } else { 0 } + if s2 == "fine" { 1 } else { 0 } + 1
    );
}
