use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize, usize); n],
    }

    for k in 1..=d {
        println!("{}", tl.iter().map(|(t, l)| t * (l + k)).max().unwrap());
    }
}
