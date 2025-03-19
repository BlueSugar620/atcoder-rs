use proconio::{input, marker::Bytes};

fn main() {
    input! {
        k: usize,
        s: Bytes,
        t: Bytes,
    }

    if s.len().abs_diff(t.len()) > k {
        println!("No");
        return;
    }

    let mut dp = vec![vec![!0 / 2; 2 * k + 1]; s.len() + 1];
    dp[0][k] = 0;
    for j in k..2 * k {
        dp[0][j + 1] = dp[0][j + 1].min(dp[0][j] + 1);
    }
    for j in 1..=k {
        dp[0][j - 1] = dp[0][j - 1].min(dp[0][j]);
    }
    for (i, &s) in s.iter().enumerate() {
        for j in 0..=2 * k {
            if k <= i + j && i + j < t.len() + k {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + if s == t[i + j - k] { 0 } else { 1 });
            }
            if k <= i + j && j > 0 {
                dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j] + 1);
            }
            if j < 2 * k {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i + 1][j] + 1);
            }
        }
    }

    println!(
        "{}",
        if dp[s.len()][t.len() + k - s.len()] <= k {
            "Yes"
        } else {
            "No"
        }
    );
}
