use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = vec![0; 100];
    for _ in 0..q {
        input! {t: u8}

        if t == 1 {
            input! {x: usize}
            stack.push(x);
        } else {
            println!("{}", stack.pop().unwrap());
        }
    }
}
