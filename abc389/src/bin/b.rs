use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let mut tmp = 1;
    for i in 1.. {
        tmp *= i;
        if tmp == x {
            println!("{}", i);
            return;
        }
    }
}
