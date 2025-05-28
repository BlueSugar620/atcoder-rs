use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    println!(
        "{}",
        a.iter()
            .map(|a| if *a > 10 { a - 10 } else { 0 })
            .sum::<u32>()
    );
}
