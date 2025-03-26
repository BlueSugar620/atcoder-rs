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
        n: usize,
        m: usize,
        l: usize,
        aa: [usize; n],
        bb: [usize; m],
        cc: [usize; l],
    }

    let a = (1 << n) - 1;
    let b = (1 << (n + m)) - 1 - a;
    let c = (1 << (n + m + l)) - 1 - a - b;
    let mut cards = vec![];
    for &a in &aa {
        cards.push(a);
    }
    for &b in &bb {
        cards.push(b);
    }
    for &c in &cc {
        cards.push(c);
    }
    let ans = is_t_win(a, b, c, n + m + l, &cards);
    if ans {
        println!("Takahashi");
    } else {
        println!("Aoki")
    }
}

#[memoise_map(a, b, c)]
fn is_t_win(a: usize, b: usize, c: usize, n: usize, cards: &[usize]) -> bool {
    let a = usize_to_vec(a, n);
    let b = usize_to_vec(b, n);
    let c = usize_to_vec(c, n);

    if a.len() == 0 {
        return false;
    }
    for (j, &ai) in a.iter().enumerate() {
        for (i, &ci) in c.iter().enumerate() {
            if cards[ai] > cards[ci] {
                let mut cc = c.clone();
                let mut aa = a.clone();
                let ind = aa.remove(j);
                aa.push(cc.remove(i));
                cc.push(ind);
                if !is_t_win(
                    vec_to_usize(b.clone()),
                    vec_to_usize(aa),
                    vec_to_usize(cc),
                    n,
                    cards,
                ) {
                    return true;
                }
            }
            let mut aa = a.clone();
            let ind = aa.remove(j);
            let mut cc = c.clone();
            cc.push(ind);
            if !is_t_win(
                vec_to_usize(b.clone()),
                vec_to_usize(aa),
                vec_to_usize(cc),
                n,
                cards,
            ) {
                return true;
            }
        }
    }
    for (i, &_) in a.iter().enumerate() {
        let mut aa = a.clone();
        let mut cc = c.clone();
        cc.push(aa.remove(i));
        if !is_t_win(
            vec_to_usize(b.clone()),
            vec_to_usize(aa),
            vec_to_usize(cc),
            n,
            cards,
        ) {
            return true;
        }
    }
    return false;
}

fn usize_to_vec(mut a: usize, n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..n {
        if a == 0 {
            break;
        }
        if a % 2 == 1 {
            res.push(i);
        }
        a /= 2;
    }
    res
}

fn vec_to_usize(a: Vec<usize>) -> usize {
    let mut res = 0;
    for &a in a.iter() {
        res += 1 << a;
    }
    res
}
