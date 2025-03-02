use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }
    println!("{}", x.max(y));
}
