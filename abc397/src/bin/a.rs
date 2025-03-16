use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    let x = (x * 10.0).round() as u16;

    if x >= 380 {
        println!("1");
    } else if x >= 375 {
        println!("2");
    } else {
        println!("3");
    }
}
