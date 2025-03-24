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

    let mut ans = 0;
    for (i, c) in s.iter().copied().enumerate() {
        if c == b'/' {
            ans = ans.max(1);
            for d in 1.. {
                if d <= i && d + i < n && s[i - d] == b'1' && s[i + d] == b'2' {
                    ans = ans.max(2 * d + 1);
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
