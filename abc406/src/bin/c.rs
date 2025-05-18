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
        n: usize,
        p: [Usize1; n],
    }

    let q = p
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { 0 })
        .collect::<Vec<_>>();
    let q = run_length_encoding(&q);
    let ans = q
        .windows(3)
        .map(|x| if x[0].0 == 1 { x[0].1 * x[2].1 } else { 0 })
        .sum::<usize>();

    println!("{}", ans);
}

pub fn run_length_encoding<T: Copy + PartialEq>(a: &[T]) -> Vec<(T, usize)> {
    let mut a = a.iter().copied().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}

pub fn run_length_decoding<T: Copy + PartialEq>(a: &[(T, usize)]) -> Vec<T> {
    a.iter()
        .map(|a| std::iter::repeat(a.0).take(a.1))
        .flatten()
        .collect::<Vec<_>>()
}
