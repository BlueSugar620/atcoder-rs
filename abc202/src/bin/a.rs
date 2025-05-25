use proconio::input;

fn main() {
    input! {
        x: [u8; 3],
    }
    println!("{}", 21 - x.iter().sum::<u8>());
}
