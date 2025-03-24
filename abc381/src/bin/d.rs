use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = HashSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let rle = run_length_encoding(&a);
    let mut set = Set::new();
    let mut que = Deque::new();
    let mut ans = 0;
    for (t, cnt) in rle.iter().copied() {
        if cnt >= 2 {
            while set.contains(&t) {
                let x = que.pop_front().unwrap();
                set.remove(&x);
            }
            que.push_back(t);
            set.insert(t);
        } else {
            set.clear();
            que.clear();
        }
        ans = ans.max(set.len());
        if cnt > 2 {
            que.clear();
            set.clear();
            que.push_back(t);
            set.insert(t);
        }
    }

    println!("{}", ans * 2);
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
