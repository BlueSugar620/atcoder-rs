use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut t: Usize1,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    }

    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();
    a.reverse();
    b.reverse();
    c.reverse();

    let mut heap = std::collections::BinaryHeap::new();
    heap.push((
        a[0] * b[0] + b[0] * c[0] + c[0] * a[0],
        0usize,
        0usize,
        0usize,
    ));

    let mut used = std::collections::HashSet::new();
    while let Some((x, i, j, k)) = heap.pop() {
        if t == 0 {
            println!("{}", x);
            return;
        } else {
            t -= 1;
            used.insert((i, j, k));
        }

        for (di, dj, dk) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let i = i.wrapping_add(di);
            let j = j.wrapping_add(dj);
            let k = k.wrapping_add(dk);
            if i.max(j).max(k) < n && !used.contains(&(i, j, k)) {
                let x = a[i] * b[j] + b[j] * c[k] + c[k] * a[i];
                heap.push((x, i, j, k));
                used.insert((i, j, k));
            }
        }
    }
}
