use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    // x - y = d
    // 3y^2d + 3yd^2 + d^3 = N
    for d in 1.. {
        if 3 * d + 3 * d * d + d * d * d > n {
            break;
        }
        let mut l = 1;
        let mut r = 1 << 30;
        while r - l > 1 {
            let o = (l + r) / 2;
            if 3 * o * o * d + 3 * o * d * d + d * d * d <= n {
                l = o;
            } else {
                r = o;
            }
        }
        if 3 * l * l * d + 3 * l * d * d + d * d * d == n {
            println!("{} {}", l + d, l);
            return;
        }
    }

    println!("-1");
}
