use proconio::{input, marker::Bytes};

const A: [u8; 2] = [b'i', b'o'];

fn main() {
    input! {
        s: Bytes,
    }

    let mut i = 0;
    let mut ans = 0;
    for &s in &s {
        if s == A[i % 2] {
            i += 1;
        } else {
            ans += 1;
        }
    }
    if i % 2 == 1 {
        ans += 1;
    }

    println!("{ans}");
}
