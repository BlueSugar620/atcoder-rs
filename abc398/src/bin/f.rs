use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    let mut pos = 0;
    let t = manacher(&s);
    for (i, t) in t.iter().enumerate().skip(n) {
        if t + i == 2 * n {
            pos = i;
            break;
        }
    }

    println!(
        "{}{}{}",
        s[..pos / 2].iter().join(""),
        if pos % 2 == 1 {
            s[pos / 2].to_string()
        } else {
            "".to_string()
        },
        s[..pos / 2].iter().rev().join("")
    )
}
pub fn manacher<T: PartialEq>(a: &[T]) -> Vec<usize> {
    if a.is_empty() {
        return vec![0];
    }
    let n = a.len();
    let mut r = vec![0; 2 * n + 1];
    let mut i = 1;
    let mut j = 1;
    while i < 2 * n {
        while j < i && i + j < 2 * n && a[(i - j) / 2 - 1] == a[(i + j) / 2] {
            j += 2;
        }
        if j == 0 {
            i += 1;
            j = 1;
            continue;
        }
        r[i] = j;

        let mut k = 1;
        while k <= i && k + r[i - k] < j {
            r[i + k] = r[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    r
}
