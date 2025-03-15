use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
    }

    let mut blocks = vec![vec![]; w];
    for (i, &(x, y)) in xy.iter().enumerate() {
        blocks[x].push((y, i));
    }
    for blocks in blocks.iter_mut() {
        blocks.sort_unstable_by_key(|x| !x.0);
    }

    let mut ans = vec![!0; n];
    while blocks.iter().all(|blocks| !blocks.is_empty()) {
        let time = blocks
            .iter()
            .map(|blocks| blocks.last().unwrap().0)
            .max()
            .unwrap();
        for blocks in blocks.iter_mut() {
            ans[blocks.pop().unwrap().1] = time;
        }
    }

    for _ in 0..q {
        input! {
            t: Usize1,
            a: Usize1,
        }

        println!("{}", if ans[a] > t { "Yes" } else { "No" });
    }
}
