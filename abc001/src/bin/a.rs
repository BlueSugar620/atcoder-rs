use proconio::input;

fn main() {
    input! {
        h: [i16; 2],
    }
    println!("{}", h[0] - h[1]);
}
