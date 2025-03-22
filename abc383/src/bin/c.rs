use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Bytes; h],
    }

    let mut que = std::collections::VecDeque::new();
    let mut used = vec![vec![!0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'H' {
                que.push_back((i, j));
                used[i][j] = 0;
            }
        }
    }

    while let Some((i, j)) = que.pop_front() {
        let dist = used[i][j];
        if d <= dist {
            continue;
        }
        for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let i = i.wrapping_add(di);
            let j = j.wrapping_add(dj);
            if i < h && j < w && s[i][j] == b'.' && used[i][j] == !0 {
                que.push_back((i, j));
                used[i][j] = dist + 1;
            }
        }
    }

    println!("{}", used.iter().flatten().filter(|x| **x < !0).count());
}
