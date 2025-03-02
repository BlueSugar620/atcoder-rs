use proconio::input;

fn main() {
    input! {
        a: [i32; 2],
        b: [i32; 2],
        c: [i32; 2],
    }

    let b = [b[0] - a[0], b[1] - a[1]];
    let c = [c[0] - a[0], c[1] - a[1]];
    println!("{}", (b[0] * c[1] - b[1] * c[0]).abs() as f64 / 2.);
}
