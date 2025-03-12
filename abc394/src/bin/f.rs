use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
        e[b].push(a);
    }
    let mut ans = 0;
    dfs(0, 0, &e, &mut ans);
    println!("{}", if ans < 5 { -1 } else { ans as i32 });
}

fn dfs(p: usize, i: usize, e: &Vec<Vec<usize>>, ans: &mut usize) -> usize {
    let mut x = e[i]
        .iter()
        .filter(|j| **j != p)
        .map(|j| dfs(i, *j, e, ans))
        .collect::<Vec<_>>();
    x.sort_unstable();
    x.reverse();
    if x.len() == 0 {
        1
    } else if x.len() < 3 {
        if *ans < 1 + x[0] {
            *ans = 1 + x[0];
        }
        1 + x[0]
    } else {
        let res = 1 + x[..3].iter().sum::<usize>();
        if 3 < x.len() {
            let tmp = res + x[3];
            if *ans < tmp {
                *ans = tmp;
            }
        }
        res
    }
}
