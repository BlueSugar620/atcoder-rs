use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        b: u64,
        p: Usize1,
        q: Usize1,
        s: [[u64; w]; h],
    }

    let mut heap = std::collections::BinaryHeap::new();
    let mut used = vec![vec![false; w]; h];
    let mut ans = s[p][q];
    used[p][q] = true;
    for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
        let p = p.wrapping_add(dx);
        let q = q.wrapping_add(dy);
        if p < h && q < w {
            heap.push((!s[p][q], p, q));
            used[p][q] = true;
        }
    }

    while let Some((d, x, y)) = heap.pop() {
        let d = !d;
        if ans <= d.saturating_mul(b) {
            println!("{}", ans);
            return;
        }
        ans += d;
        for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let x = x.wrapping_add(dx);
            let y = y.wrapping_add(dy);
            if x < h && y < w && !used[x][y] {
                heap.push((!s[x][y], x, y));
                used[x][y] = true;
            }
        }
    }

    println!("{}", ans);
}
