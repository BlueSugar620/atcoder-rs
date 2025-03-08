use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i.min(j).min(n - 1 - i).min(n - 1 - j) % 2 == 0 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", ans.iter().map(|ans| ans.iter().join("")).join("\n"));
}
