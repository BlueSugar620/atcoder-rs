use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }

    println!(
        "{}",
        if s.len() & 1 == 1
            && s[..s.len() / 2].iter().all(|c| *c == b'1')
            && s[s.len() / 2] == b'/'
            && s[s.len() / 2 + 1..].iter().all(|c| *c == b'2')
        {
            "Yes"
        } else {
            "No"
        }
    );
}
