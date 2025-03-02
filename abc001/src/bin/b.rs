use proconio::input;

fn main() {
    input! {
        m: u32,
    }

    if m < 100 {
        println!("00");
    } else if m < 1000 {
        println!("0{}", m / 100);
    } else if m <= 5000 {
        println!("{}", m / 100);
    } else if m <= 30000 {
        println!("{}", m / 1000 + 50);
    } else if m <= 70000 {
        println!("{}", (m / 1000 - 30) / 5 + 80);
    } else {
        println!("89");
    }
}
