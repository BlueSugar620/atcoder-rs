use proconio::input;

fn main() {
    input! {
        x: [u8; 3],
    }
    if x[0] != x[1] && x[1] != x[2] && x[2] != x[0] {
        println!("0");
    } else {
        println!("{}", x.iter().fold(0, |acc, a| acc ^ a));
    }
}
