use itertools::Itertools;
use proconio::input;

const M: usize = 1_000_000;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let sieve = SieveOfEratosthenes::new(M + 1);
    let primes = sieve.primes(M + 1);
    let mut cnt = vec![0; M + 1];
    for &a in &a {
        cnt[a] += 1;
    }
    for p in &primes {
        for i in (1..=M / p).rev() {
            cnt[i] += cnt[i * p];
        }
    }
    let mut ans = (0..M + 1)
        .map(|i| if cnt[i] >= k { i } else { 0 })
        .collect::<Vec<_>>();
    for p in &primes {
        for i in 1..=M / p {
            ans[i * p] = ans[i * p].max(ans[i]);
        }
    }
    println!("{}", a.iter().map(|a| ans[*a]).join("\n"));
}

pub struct SieveOfEratosthenes {
    min_factor: Vec<usize>,
}

impl SieveOfEratosthenes {
    pub fn new(n: usize) -> Self {
        let mut min_factor = (0..n).collect::<Vec<_>>();
        for i in 2..n {
            if min_factor[i] == i {
                for j in (2..).take_while(|j| i * j < n) {
                    if min_factor[i * j] == i * j {
                        min_factor[i * j] = i;
                    }
                }
            }
        }
        Self { min_factor }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        n != 0 && n != 1 && self.min_factor[n] == n
    }

    pub fn primes(&self, n: usize) -> Vec<usize> {
        (0..n).filter(|i| self.is_prime(*i)).collect::<Vec<_>>()
    }

    pub fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        if n == 1 {
            return vec![(1, 1)];
        }
        let mut res = vec![];
        while n > 1 {
            let p = self.min_factor[n];
            let mut cnt = 0;
            while self.min_factor[n] == p {
                n /= p;
                cnt += 1;
            }
            res.push((p, cnt));
        }
        res
    }

    pub fn divisors(&self, n: usize) -> Vec<usize> {
        if n == 1 {
            return vec![1];
        }
        let factorize = self.factorize(n);
        let mut res = vec![1];
        for &(p, cnt) in &factorize {
            for i in 0..res.len() {
                let mut tmp = 1;
                for _ in 0..cnt {
                    tmp *= p;
                    res.push(res[i] * tmp);
                }
            }
        }
        res
    }
}
