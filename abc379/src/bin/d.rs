use itertools::*;
use memoise::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        q: usize,
    }

    let mut x = 0;
    let mut v = Deque::new();

    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            v.push_back(x);
        } else if t == 2 {
            input! {
                t: u64,
            }
            x += t;
        } else {
            input! {
                h: u64,
            }
            let mut ans = 0;
            while x - v.front().unwrap_or(&x) >= h {
                v.pop_front();
                ans += 1;
            }
            println!("{}", ans);
        }
    }
}
