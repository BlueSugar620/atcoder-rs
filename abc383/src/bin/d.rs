use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let primes = SieveOfEratosthenesMini::new(1 << 21).primes();
    let mut ans = 0usize;
    for i in 0.. {
        if primes[i].pow(8) > n {
            break;
        }
        ans += 1;
    }

    for i in 1.. {
        if primes[i] * primes[i] * 2 * 2 > n {
            break;
        }
        for j in 0..i {
            if primes[i] * primes[i] * primes[j] * primes[j] > n {
                break;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}

pub struct SieveOfEratosthenesMini {
    is_prime: Vec<u64>,
}

impl SieveOfEratosthenesMini {
    pub fn new(n: usize) -> Self {
        let mut is_prime = vec![!0; (n + 63) / 64];
        is_prime[0] = !0 ^ 1 ^ 2;
        for i in n % 64..64 {
            is_prime[(n + 63) / 64 - 1] ^= 1 << i;
        }
        for i in 2..n {
            if (is_prime[i / 64] >> (i % 64)) & 1 == 1 {
                for j in (2..).take_while(|j| i * j < n) {
                    if (is_prime[i * j / 64] >> (i * j % 64)) & 1 == 1 {
                        is_prime[i * j / 64] = is_prime[i * j / 64] ^ (1 << (i * j % 64));
                    }
                }
            }
        }
        Self { is_prime }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        (self.is_prime[n / 64] >> (n % 64)) & 1 == 1
    }

    pub fn primes(&self) -> Vec<usize> {
        let mut res = vec![];
        for (i, &x) in self.is_prime.iter().enumerate() {
            if x != 0 {
                for j in 0..64 {
                    if (x >> j) & 1 == 1 {
                        res.push(i * 64 + j);
                    }
                }
            }
        }
        res
    }
}
