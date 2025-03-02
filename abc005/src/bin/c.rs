use proconio::input;

fn main() {
    input! {
        t: u8,
        n: usize,
        a: [u8; n],
        m: usize,
        b: [u8; m],
    }

    let mut idx = 0;
    for &b in &b {
        while idx < n && a[idx] + t < b {
            idx += 1;
        }
        if idx < n && a[idx] <= b {
            idx += 1;
        } else {
            println!("no");
            return;
        }
    }
    println!("yes");
}
