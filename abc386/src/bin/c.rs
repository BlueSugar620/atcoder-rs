use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _k: u8,
        s: Bytes,
        t: Bytes,
    }

    if s.len() + 1 == t.len() {
        let x = s
            .iter()
            .zip(t.iter())
            .position(|(s, t)| s != t)
            .unwrap_or(s.len());
        if s[..x] == t[..x] && s[x..] == t[x + 1..] {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if s.len() == t.len() + 1 {
        let x = t
            .iter()
            .zip(s.iter())
            .position(|(t, s)| t != s)
            .unwrap_or(t.len());
        if t[..x] == s[..x] && t[x..] == s[x + 1..] {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if s.len() == t.len() {
        if s.iter().zip(t.iter()).filter(|(s, t)| s != t).count() <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
