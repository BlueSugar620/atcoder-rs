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
        s: Chars,
        q: usize,
        k: [Usize1; q],
    }

    for &k in &k {
        let n = k / s.len();
        let r = k % s.len();
        print!(
            "{} ",
            if (n.count_ones() & 1 == 0) == s[r].is_ascii_uppercase() {
                s[r].to_ascii_uppercase()
            } else {
                s[r].to_ascii_lowercase()
            }
        );
    }

    println!("");
}
