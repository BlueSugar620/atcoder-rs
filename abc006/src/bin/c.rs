use proconio::input;

fn main() {
    input! {
        n: isize,
        m: isize,
    }

    for a in 0..=n {
        let c = m - 2 * a - 3 * (n - a);
        let b = n - a - c;
        if 0 <= a && 0 <= b && 0 <= c {
            println!("{} {} {}", a, b, c);
            return;
        }
    }

    println!("-1 -1 -1");
}
