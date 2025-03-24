use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec!['-'; n];
    ans[n / 2] = '=';
    ans[(n - 1) / 2] = '=';

    println!("{}", ans.iter().join(""));
}
