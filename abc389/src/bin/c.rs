use proconio::{input, marker::Usize1};

fn main() {
    input! {
        q: usize,
    }

    let mut snake = vec![0];
    let mut pos = 0;

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                l: u64,
            }
            snake.push(snake.last().unwrap() + l);
        } else if t == 2 {
            pos += 1;
        } else {
            input! {
                k: Usize1,
            }

            println!("{}", snake[k + pos] - snake[pos]);
        }
    }
}
