use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }
    let certainly = (0..=9).filter(|i| s[*i] == b'o').collect::<Vec<_>>();
    let probably = (0..=9)
        .filter(|i| s[*i] == b'?' || s[*i] == b'o')
        .collect::<Vec<_>>();
    let ans = (0..=9999)
        .filter(|i| {
            let mut i = *i;
            let mut v = vec![];
            for _ in 0..4 {
                v.push(i % 10);
                i /= 10;
            }
            certainly.iter().all(|i| v.contains(i)) && v.iter().all(|i| probably.contains(i))
        })
        .count();
    println!("{}", ans);
}
