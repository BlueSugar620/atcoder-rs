use proconio::input;

const D: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
fn main() {
    input! {
        d: String,
    }

    let x = D.iter().position(|x| *x == d).unwrap();
    println!("{}", D[(x + 4) % 8]);
}
