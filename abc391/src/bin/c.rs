use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut pos = (0..n).collect::<Vec<_>>();
    let mut cnt = vec![1; n];
    let mut ans = 0;

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                p: Usize1,
                h: Usize1,
            }
            let x = pos[p];
            pos[p] = h;
            cnt[x] -= 1;
            cnt[h] += 1;
            if cnt[x] == 1 {
                ans -= 1;
            }
            if cnt[h] == 2 {
                ans += 1;
            }
        } else {
            println!("{}", ans);
        }
    }
}
