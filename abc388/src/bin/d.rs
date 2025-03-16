use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut imos = vec![0; n + 1];
    for (i, a) in a.iter_mut().enumerate() {
        *a = (*a).wrapping_add(imos[i]);
        let d = (*a).min(n - i - 1);
        *a -= d;
        imos[i + 1] = imos[i + 1].wrapping_add(imos[i]).wrapping_add(1);
        imos[i + 1 + d] = imos[i + 1 + d].wrapping_sub(1);
    }
    println!("{}", a.iter().join(" "));
}
