use itertools::*;
use memoise::*;
use num::integer::Roots;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        q: usize,
        ask: [usize; q]
    }

    let sieve = SieveOfEratosthenes::new(10usize.pow(6) + 100);
    let mut ans = vec![];
    for i in 2..=10usize.pow(6) {
        let fact = sieve.factorize(i);
        if fact.len() == 2 {
            ans.push(i * i);
        }
    }

    for a in &ask {
        let x = ans.upper_bound(a);
        println!("{}", ans[x - 1]);
    }
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
