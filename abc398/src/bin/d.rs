use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        mut r: isize,
        mut c: isize,
        s: Bytes,
    }

    let mut set = std::collections::HashSet::new();
    let (mut x, mut y) = (0, 0);
    set.insert((x, y));

    for &s in &s {
        let (dx, dy) = match s {
            b'N' => (1, 0),
            b'W' => (0, 1),
            b'S' => (-1, 0),
            b'E' => (0, -1),
            _ => panic!(),
        };
        x += dx;
        y += dy;
        set.insert((x, y));
        r += dx;
        c += dy;
        if set.contains(&(r, c)) {
            print!("1");
        } else {
            print!("0");
        }
    }

    println!("");
}
