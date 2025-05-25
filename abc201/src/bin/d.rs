use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h],
    }

    let mut memo = vec![vec![std::isize::MIN; w]; h];
    let point = negamax(0, 0, &mut memo, &a);
    println!(
        "{}",
        if point > 0 {
            "Takahashi"
        } else if point < 0 {
            "Aoki"
        } else {
            "Draw"
        }
    );
}

fn negamax(i: usize, j: usize, memo: &mut Vec<Vec<isize>>, a: &Vec<Vec<u8>>) -> isize {
    if memo[i][j] != std::isize::MIN {
        return memo[i][j];
    }
    let h = a.len();
    let w = a[0].len();
    if (i, j) == (h - 1, w - 1) {
        memo[i][j] = 0;
        return 0;
    }
    let mut point = std::isize::MIN;
    for (di, dj) in [(1, 0), (0, 1)] {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if ni < h && nj < w {
            let tmp = if a[ni][nj] == b'+' { 1 } else { -1 } - negamax(ni, nj, memo, a);
            if point < tmp {
                point = tmp;
            }
        }
    }
    memo[i][j] = point;
    point
}
