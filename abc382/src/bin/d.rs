use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut ans = vec![];
    dfs(&mut vec![], n, m, &mut ans);
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|ans| ans.iter().join(" ")).join("\n"));
}

fn dfs(a: &mut Vec<u32>, n: usize, m: u32, ans: &mut Vec<Vec<u32>>) {
    if a.len() == n {
        ans.push(a.clone());
        return;
    }

    let l = a.len();

    for x in a.last().unwrap_or(&9u32.wrapping_neg()).wrapping_add(10)..=m {
        if x + (n - l - 1) as u32 * 10 > m {
            break;
        }
        a.push(x);
        dfs(a, n, m, ans);
        a.pop();
    }
}
