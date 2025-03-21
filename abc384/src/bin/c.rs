use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x: [u32; 5],
    }

    let mut ans = (1..1 << 5)
        .map(|v| {
            let idx = (0..5).filter(|i| (v >> i) & 1 == 1).collect::<Vec<_>>();
            let p = idx.iter().map(|i| x[*i]).sum::<u32>();
            let s = idx
                .iter()
                .map(|i| (*i as u8 + b'A') as char)
                .collect::<String>();
            (p, s)
        })
        .collect::<Vec<_>>();
    ans.sort_unstable_by_key(|x| (!x.0, x.1.clone()));
    println!("{}", ans.iter().map(|x| x.1.clone()).join("\n"));
}
