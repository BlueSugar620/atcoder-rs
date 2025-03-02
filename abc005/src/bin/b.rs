use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u8; n],
    }

    println!("{}", t.iter().min().unwrap());
}
