use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xy: [(usize, usize); m],
    }
    xy.sort_unstable();
    let mut now = BTreeSet::new();
    let mut next_add = BTreeSet::new();
    let mut next_remove = BTreeSet::new();
    let mut row = 0;
    now.insert(n);
    for &(x, y) in &xy {
        if row != x {
            for i in next_remove.iter() {
                now.remove(i);
            }
            next_remove.clear();
            for i in next_add.iter() {
                now.insert(*i);
            }
            next_add.clear();
        }
        next_remove.insert(y);
        if 0 < y && now.contains(&(y - 1)) {
            next_add.insert(y);
        }
        if y < 2 * n - 1 && now.contains(&(y + 1)) {
            next_add.insert(y);
        }
        row = x;
    }
    for i in next_remove.iter() {
        now.remove(i);
    }
    next_remove.clear();
    for i in next_add.iter() {
        now.insert(*i);
    }
    next_add.clear();

    println!("{}", now.len());
}
