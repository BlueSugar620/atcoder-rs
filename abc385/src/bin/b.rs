use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: Usize1,
        mut y: Usize1,
        s: [Bytes; h],
        t: Bytes,
    }

    let mut house = std::collections::HashSet::new();
    for &t in &t {
        let (dx, dy) = match t {
            b'U' => (!0, 0),
            b'D' => (1, 0),
            b'L' => (0, !0),
            b'R' => (0, 1),
            _ => (0, 0),
        };
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        if nx < h && ny < w && s[nx][ny] != b'#' {
            x = nx;
            y = ny;
            if s[x][y] == b'@' {
                house.insert((x, y));
            }
        }
    }

    println!("{} {} {}", x + 1, y + 1, house.len());
}
