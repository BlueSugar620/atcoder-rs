use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        mut a: Bytes,
    }

    let mut b = a.clone();
    while b.len() > 1 {
        b = b
            .chunks(3)
            .map(|b| {
                if b.iter().filter(|b| **b == b'0').count() > 1 {
                    b'0'
                } else {
                    b'1'
                }
            })
            .collect::<Vec<_>>();
    }

    if b == vec![b'0'] {
        a = a
            .iter()
            .map(|a| if *a == b'0' { b'1' } else { b'0' })
            .collect::<Vec<_>>();
    }

    let ans = dfs(&a);
    println!("{}", ans);
}

fn dfs(a: &[u8]) -> usize {
    if a.len() == 1 {
        return if a[0] == b'0' { 0 } else { 1 };
    }
    let n = a.len() / 3;
    let x = dfs(&a[..n]);
    let y = dfs(&a[n..2 * n]);
    let z = dfs(&a[2 * n..]);
    (x + y).min(y + z).min(z + x)
}
