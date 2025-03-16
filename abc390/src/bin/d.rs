use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = std::collections::HashSet::new();
    for x in bells(n, &a).iter() {
        let x = x.iter().fold(0, |acc, a| acc ^ a);
        ans.insert(x);
    }
    println!("{}", ans.len());
}

fn bells(n: usize, a: &[u64]) -> Vec<Vec<u64>> {
    let mut dp = vec![vec![a[0]]];
    for i in 1..n {
        let mut ndp = vec![];
        for v in dp.iter() {
            let mut w = v.clone();
            for k in 0..w.len() {
                w[k] += a[i];
                ndp.push(w.clone());
                w[k] -= a[i];
            }
            w.push(a[i]);
            ndp.push(w.clone());
            w.pop();
        }
        dp = ndp;
    }
    dp
}
