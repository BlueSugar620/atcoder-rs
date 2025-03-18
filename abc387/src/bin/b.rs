use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        x: u16,
    }

    println!(
        "{}",
        iproduct!(1..=9, 1..=9)
            .map(|(i, j)| if i * j == x { 0 } else { i * j })
            .sum::<u16>()
    );
}
