use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        r: f64,
    }

    let idx = (1..r as usize).collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..r as usize {
        ans += idx.lower_bound_by(|x| {
            ((*x as f64 + 0.5) * (*x as f64 + 0.5) + (i as f64 + 0.5) * (i as f64 + 0.5))
                .partial_cmp(&(r * r))
                .unwrap()
        });
    }

    println!("{}", ans * 4 + 1);
}
