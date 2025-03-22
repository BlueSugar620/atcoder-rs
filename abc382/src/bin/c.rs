use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut v = vec![!0; 1 << 20 + 1];
    for (i, &a) in a.iter().enumerate().rev() {
        v[a] = i + 1;
    }
    for i in 0..1 << 20 {
        v[i + 1] = v[i + 1].min(v[i]);
    }
    for &b in &b {
        println!("{}", v[b] as isize);
    }
}
