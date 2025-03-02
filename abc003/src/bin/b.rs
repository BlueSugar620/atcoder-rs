use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    for (s, t) in s.chars().zip(t.chars()) {
        if s == t {
            continue;
        }
        if "atcoder".contains(s) && t == '@' {
            continue;
        }
        if "atcoder".contains(t) && s == '@' {
            continue;
        }
        println!("You will lose");
        return;
    }
    println!("You can win")
}
