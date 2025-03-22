use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Bytes; h],
    }

    let mut ans = 0;

    for x in 0..h {
        for y in 0..w {
            for z in 0..h {
                for v in 0..w {
                    if s[x][y] == b'#' || s[z][v] == b'#' {
                        continue;
                    }
                    let mut tmp = 0;
                    for i in 0..h {
                        for j in 0..w {
                            if s[i][j] == b'.'
                                && (x.abs_diff(i) + y.abs_diff(j) <= d
                                    || z.abs_diff(i) + v.abs_diff(j) <= d)
                            {
                                tmp += 1;
                            }
                        }
                    }
                    ans = ans.max(tmp);
                }
            }
        }
    }

    println!("{}", ans);
}
