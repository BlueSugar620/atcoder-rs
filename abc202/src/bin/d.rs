use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: Usize1,
    }
    let meru_prastaar = MeruPrastaar::new(a + b);
    let mut ans = vec![];
    for _ in 0..a + b {
        let cnt = meru_prastaar.binom(a + b - 1, b);
        if k >= cnt {
            ans.push('b');
            k -= cnt;
            b -= 1;
        } else {
            ans.push('a');
            a -= 1;
        }
    }
    println!("{}", ans.iter().join(""));
}

pub struct MeruPrastaar {
    n: usize,
    values: Box<[usize]>,
}

impl MeruPrastaar {
    pub fn new(n: usize) -> Self {
        let mut values = vec![0usize; (n + 1) * (n + 1)];
        values[0] = 1;
        for i in 1..=n {
            values[i * (n + 1)] = 1;
            for j in 1..=i {
                values[i * (n + 1) + j] =
                    values[(i - 1) * (n + 1) + j - 1] + values[(i - 1) * (n + 1) + j];
            }
        }
        Self {
            n,
            values: values.into_boxed_slice(),
        }
    }

    pub fn binom(&self, n: usize, r: usize) -> usize {
        self.values[n * (self.n + 1) + r]
    }
}
