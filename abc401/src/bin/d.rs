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

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }

    if s.iter().filter(|c| **c == 'o').count() == k {
        println!(
            "{}",
            s.iter().map(|c| if *c == '?' { '.' } else { *c }).join("")
        );
        return;
    }

    for i in 0..n {
        if s[i] == '?' && ((i > 0 && s[i - 1] == 'o') || (i + 1 < n && s[i + 1] == 'o')) {
            s[i] = '.';
        }
    }

    let rle = run_length_encoding(&s);
    if rle
        .iter()
        .map(|&(c, i)| {
            if c == 'o' {
                i
            } else if c == '.' {
                0
            } else {
                (i + 1) / 2
            }
        })
        .sum::<usize>()
        == k
    {
        let mut ans = vec![];
        for &(c, i) in &rle {
            if c == 'o' || c == '.' {
                for _ in 0..i {
                    ans.push(c);
                }
            } else {
                if i & 1 == 1 {
                    ans.push('o');
                    for _ in 0..i / 2 {
                        ans.push('.');
                        ans.push('o');
                    }
                } else {
                    for _ in 0..i {
                        ans.push('?');
                    }
                }
            }
        }
        println!("{}", ans.iter().join(""));
        return;
    }

    println!("{}", s.iter().join(""));
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
