use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use rand::{thread_rng, Rng};
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;
type Heap<T> = BinaryHeap<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: Usize1,
    }

    let meru = MeruPrastaar::new(a + b);

    let mut ans = vec![];
    for _ in 0..a + b {
        if a == 0 {
            ans.push('b');
            b -= 1;
            continue;
        }
        if b == 0 {
            ans.push('a');
            a -= 1;
            continue;
        }
        // `b` が先頭にくる中で最小のindex
        let x = meru.binom(a + b - 1, b);
        if k >= x {
            k -= x;
            ans.push('b');
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
