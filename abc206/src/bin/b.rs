use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 0.. {
        ans += i;
        if n <= ans {
            println!("{}", i);
            return;
        }
    }
}
