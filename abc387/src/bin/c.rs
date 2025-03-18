use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l: Usize1,
        r: usize,
    }

    println!("{}", f(r) - f(l));
}

fn f(x: usize) -> usize {
    let xs = x
        .to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<_>>();
    let d = x.to_string().len();

    let mut ans = 0;
    if xs[1..].iter().all(|x| xs[0] > *x) {
        ans += 1;
    }
    for k in 1..d {
        if xs[1..k].iter().all(|x| xs[0] > *x) {
            ans += xs[0].min(xs[k]) * xs[0].pow((d - k - 1) as u32);
        }
    }
    ans += (1..xs[0]).map(|x| x.pow((d - 1) as u32)).sum::<usize>();
    for k in 1..d {
        ans += (1usize..10).map(|x| x.pow((k - 1) as u32)).sum::<usize>();
    }
    ans
}
