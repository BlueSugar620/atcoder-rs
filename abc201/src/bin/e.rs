use proconio::{input, marker::Usize1};

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, u64); n - 1],
    }
    let mut e = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    let mut stack = vec![0];
    let mut dist = vec![!0; n];
    dist[0] = 0;
    while let Some(u) = stack.pop() {
        for &(v, w) in &e[u] {
            if dist[v] == !0 {
                dist[v] = dist[u] ^ w;
                stack.push(v);
            }
        }
    }

    let mut bitcnt = vec![0; 64];
    for (i, bitcnt) in bitcnt.iter_mut().enumerate() {
        for &d in &dist {
            *bitcnt += (d >> i) & 1;
        }
    }
    let ans = bitcnt
        .iter()
        .enumerate()
        .map(|(i, b)| b * (n as u64 - b) * ((1 << i) % MOD) % MOD)
        .fold(0, |acc, a| (acc + a) % MOD);
    println!("{}", ans);
}
