use proconio::input;

fn main() {
    input! {
        l: u64,
        mut r: u64,
    }
    r += 1;
    let mut h = vec![0; r as usize];
    for i in 1..r {
        let x = (r - 1) / i - (l - 1) / i;
        h[i as usize] = x * x;
    }
    let mut f = calc_f(&h);
    for i in l..r {
        f[i as usize] -= 2 * ((r - 1) / i) - 1;
    }
    println!("{}", f[2..].iter().sum::<u64>());
}
fn calc_f(h: &[u64]) -> Vec<u64> {
    let n = h.len();
    let mut f = vec![0; n];
    for i in (1..n).rev() {
        f[i] = h[i]
            - (2..)
                .take_while(|j| i * j < n)
                .map(|j| f[i * j])
                .sum::<u64>();
    }
    f
}
