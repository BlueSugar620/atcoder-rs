use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let rle = run_length_encoding(&a);
    println!(
        "{}",
        n * (n - 1) / 2
            - rle
                .iter()
                .map(|(_, x)| if *x == 0 { 0 } else { x * (x - 1) / 2 })
                .sum::<usize>()
    );
}
pub fn run_length_encoding<T: Copy + PartialEq>(a: &[T]) -> Vec<(T, usize)> {
    let mut a = a.iter().copied().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}

pub fn run_length_decoding<T: Copy + PartialEq>(a: &[(T, usize)]) -> Vec<T> {
    a.iter()
        .map(|a| std::iter::repeat(a.0).take(a.1))
        .flatten()
        .collect::<Vec<_>>()
}
