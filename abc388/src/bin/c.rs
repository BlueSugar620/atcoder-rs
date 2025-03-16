use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let ans = a
        .iter()
        .map(|ai| n - a.partition_point(|x| *x < 2 * ai))
        .sum::<usize>();
    println!("{}", ans);
}
