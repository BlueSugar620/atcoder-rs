use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: i32,
        da: [(u8, i32); n],
    }

    for &(d, a) in &da {
        if d == 1 {
            if 1600 <= r && r < 2800 {
                r += a;
            }
        } else {
            if 1200 <= r && r < 2400 {
                r += a;
            }
        }
    }
    println!("{}", r);
}
