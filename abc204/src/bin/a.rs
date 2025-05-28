use proconio::input;

fn main() {
    input! {
        x: [u8; 2],
    }
    if x[0] == x[1] {
        println!("{}", x[0]);
    } else {
        println!("{}", 3 - x.iter().sum::<u8>());
    }
}
