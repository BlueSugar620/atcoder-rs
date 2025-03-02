use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        se: [Bytes; n],
    }

    let mut imos = vec![0i64; 60 * 24 / 5 + 2];
    for se in &se {
        let s = read_time(se[..4].to_vec());
        let e = read_time(se[5..].to_vec());
        let idx_s = (s.0 * 60 + s.1) / 5;
        let idx_e = (e.0 * 60 + e.1 + 4) / 5;
        imos[idx_s] += 1;
        imos[idx_e] -= 1;
    }
    let mut cnt = 0;
    let mut is_rain = vec![false; 60 * 24 / 5 + 1];
    for (b, c) in is_rain.iter_mut().zip(imos.iter()) {
        cnt += c;
        if cnt > 0 {
            *b = true;
        }
    }

    let mut ans = vec![];
    let mut now = is_rain.iter().position(|b| *b).unwrap();
    for i in now..=60 * 24 / 5 {
        if is_rain[i] {
            if now == 300 {
                now = i;
            }
        } else {
            if now != 300 {
                ans.push((now, i));
                now = 300;
            } else {
                now = 300;
            }
        }
    }
    if is_rain[60 * 24 / 5] {
        ans.push((now, 60 * 24 / 5 + 1));
    }

    for &ans in &ans {
        let s = (ans.0 / 12, ans.0 % 12 * 5);
        let e = (ans.1 / 12, ans.1 % 12 * 5);
        println!("{:>02}{:>02}-{:>02}{:>02}", s.0, s.1, e.0, e.1);
    }
}

fn read_time(time: Vec<u8>) -> (usize, usize) {
    let time = time
        .iter()
        .map(|&t| (t - b'0') as usize)
        .collect::<Vec<_>>();
    (time[0] * 10 + time[1], time[2] * 10 + time[3])
}
