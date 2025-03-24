use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: [u8; 7],
    }

    let mut a = a.iter().counts().iter().map(|x| *x.1).collect_vec();
    a.sort_unstable();
    a.reverse();
    println!(
        "{}",
        if a.len() > 1 && a[0] >= 3 && a[1] >= 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
