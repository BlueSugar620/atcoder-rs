use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut xyc: [(Usize1, Usize1, char); m],
    }

    xyc.sort_unstable_by_key(|z| (!z.1, z.2));
    let mut a = 0;
    for &(x, _y, c) in &xyc {
        if c == 'W' {
            if x < a {
                println!("No");
                return;
            }
        } else {
            a = a.max(x + 1);
        }
    }

    println!("Yes");
}
