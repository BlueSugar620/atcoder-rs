use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        s: Bytes,
    }

    let mut t = s.clone();
    t.sort_unstable();
    t.dedup();

    println!(
        "{}",
        if s.chunks(2).all(|s| s.len() == 2 && s[0] == s[1]) && t.len() * 2 == s.len() {
            "Yes"
        } else {
            "No"
        }
    );
}
