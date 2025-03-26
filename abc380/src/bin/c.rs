use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        k: Usize1,
        s: Chars,
    }

    let k = 2 * k + if s[0] == '0' { 1 } else { 0 };
    let mut rle = run_length_encoding(&s);
    rle[k - 2].1 += rle[k].1;
    rle[k].1 = 0;
    println!("{}", run_length_decoding(&rle).iter().join(""));
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
