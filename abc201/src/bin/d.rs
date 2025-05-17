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
type Heap<T> = BinaryHeap<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h],
    }

    let mut memo = vec![vec![None; w]; h];
    let pt = negamax((0, 0), &a, &mut memo);

    println!(
        "{}",
        if pt > 0 {
            "Takahashi"
        } else if pt == 0 {
            "Draw"
        } else {
            "Aoki"
        }
    );
}

/// 状態 `State` から自分の手番としてゲームを始める。
/// (自分の得点) - (相手の得点) を最大化するように遷移する。
fn negamax(state: (usize, usize), a: &Vec<Vec<u8>>, memo: &mut Vec<Vec<Option<i64>>>) -> i64 {
    if let Some(pt) = memo[state.0][state.1] {
        return pt;
    }

    let h = a.len();
    let w = a[0].len();
    if state == (h - 1, w - 1) {
        return 0;
    }

    let mut next_states = vec![];
    if state.0 < h - 1 {
        next_states.push((state.0 + 1, state.1));
    }
    if state.1 < w - 1 {
        next_states.push((state.0, state.1 + 1));
    }
    let next = next_states
        .iter()
        .map(|next_state| if a[next_state.0][next_state.1] == b'+' { 1 } else { -1 } - negamax(*next_state, a,  memo))
        .max()
        .unwrap();
    memo[state.0][state.1] = Some(next);
    next
}
