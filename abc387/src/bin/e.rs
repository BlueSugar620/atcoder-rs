use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    }

    if n.len() < 6 {
        let n = n
            .iter()
            .map(|i| (i - b'0') as usize)
            .fold(0, |acc, a| 10 * acc + a);
        for x in n..2 * n {
            if x % x
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as usize)
                .sum::<usize>()
                == 0
                && (x + 1)
                    % (x + 1)
                        .to_string()
                        .chars()
                        .map(|c| (c as u8 - b'0') as usize)
                        .sum::<usize>()
                    == 0
            {
                println!("{}", x);
                return;
            }
        }
        println!("-1");
    } else {
        let m = (n[0] - b'0') as usize * 10 + (n[1] - b'0') as usize;
        if 10 <= m && m < 17 {
            println!("17{}", vec![0; n.len() - 2].iter().join(""));
        } else if m < 26 {
            println!("26{}", vec![0; n.len() - 2].iter().join(""));
        } else if m < 35 {
            println!("35{}", vec![0; n.len() - 2].iter().join(""));
        } else if m < 62 {
            println!("62{}", vec![0; n.len() - 2].iter().join(""));
        } else {
            println!("107{}", vec![0; n.len() - 2].iter().join(""));
        }
    }
}
