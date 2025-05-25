use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }
    let mut freq = vec![0; n];
    for &a in &a {
        freq[a] += 1;
    }
    let ans = c.iter().map(|c| freq[b[*c]]).sum::<usize>();
    println!("{}", ans);
}
