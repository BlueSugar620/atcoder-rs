use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut sx: i64,
        mut sy: i64,
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    }

    let mut x_set = std::collections::BTreeMap::new();
    let mut y_set = std::collections::BTreeMap::new();
    for &(x, y) in &xy {
        x_set
            .entry(x)
            .or_insert(std::collections::BTreeSet::new())
            .insert(y);
        y_set
            .entry(y)
            .or_insert(std::collections::BTreeSet::new())
            .insert(x);
    }

    let mut ans = 0usize;

    'LOOP: for &(d, c) in &dc {
        let (dx, dy) = match d {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(),
        };
        let (nx, ny) = (sx + c * dx, sy + c * dy);
        if dy == 0 {
            if !y_set.contains_key(&sy) {
                (sx, sy) = (nx, ny);
                continue 'LOOP;
            }
            let a = sx.min(nx);
            let b = sx.max(nx);
            let v = y_set
                .get(&sy)
                .unwrap()
                .range(a..=b)
                .copied()
                .collect::<Vec<_>>();
            ans += v.len();
            for x in &v {
                x_set.get_mut(x).unwrap().remove(&sy);
                y_set.get_mut(&sy).unwrap().remove(x);
            }
        } else {
            if !x_set.contains_key(&sx) {
                (sx, sy) = (nx, ny);
                continue 'LOOP;
            }
            let a = sy.min(ny);
            let b = sy.max(ny);
            let v = x_set
                .get(&sx)
                .unwrap()
                .range(a..=b)
                .copied()
                .collect::<Vec<_>>();
            ans += v.len();
            for y in &v {
                x_set.get_mut(&sx).unwrap().remove(y);
                y_set.get_mut(y).unwrap().remove(&sx);
            }
        }
        (sx, sy) = (nx, ny);
    }

    println!("{} {} {}", sx, sy, ans);
}
