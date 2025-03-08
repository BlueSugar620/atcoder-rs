use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut hato = (0..n).collect::<Vec<_>>();
    let mut su = (0..n).collect::<Vec<_>>();
    let mut inv_su = (0..n).collect::<Vec<_>>();

    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! {
                a: Usize1,
                b: Usize1,
            }

            hato[a] = su[b];
        } else if t == 2 {
            input! {
                a: Usize1,
                b: Usize1,
            }

            inv_su.swap(su[a], su[b]);
            su.swap(a, b);
        } else {
            input! {
                a: Usize1,
            }

            println!("{}", inv_su[hato[a]] + 1);
        }
    }
}
