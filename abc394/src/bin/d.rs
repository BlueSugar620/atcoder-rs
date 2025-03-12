use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }

    let mut stack = vec![];
    'LOOP: for &c in &s {
        for (a, b) in [(b'(', b')'), (b'<', b'>'), (b'[', b']')] {
            if c == a {
                stack.push(c);
                continue 'LOOP;
            } else if c == b {
                if stack.len() > 0 && *stack.last().unwrap() == a {
                    stack.pop();
                    continue 'LOOP;
                } else {
                    stack.push(c);
                    continue 'LOOP;
                }
            }
        }
    }

    println!("{}", if stack.len() > 0 { "No" } else { "Yes" });
}
