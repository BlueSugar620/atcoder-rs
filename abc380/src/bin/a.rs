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
        mut n: Bytes,
    }
    n.sort_unstable();
    println!(
        "{}",
        if n == vec![b'1', b'2', b'2', b'3', b'3', b'3'] {
            "Yes"
        } else {
            "No"
        }
    );
}
