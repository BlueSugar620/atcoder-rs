use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h],
    }

    let b = a
        .iter()
        .map(|a| {
            a.iter()
                .enumerate()
                .map(|(i, a)| ((a - b'0') as u64) << i)
                .sum::<u64>()
        })
        .collect::<Vec<_>>();

    let g = (0..1 << w)
        .map(|x: u64| (x.count_ones() as u64).min(w as u64 - x.count_ones() as u64))
        .collect::<Vec<_>>();
    let mut h = vec![0; 1 << w];
    for &b in &b {
        h[b as usize] += 1;
    }

    let ans = bitwise_convolution::<Xor>(&g, &h);
    println!("{}", ans[..1 << w].iter().min().unwrap());
}

const MOD: u64 = 998_244_353;

enum Xor {}
impl BitOp for Xor {
    type Value = u64;
    fn mul(a: &Self::Value, b: &Self::Value) -> Self::Value {
        a * b % MOD
    }
    fn sqrt_coef(a: &mut Self::Value) {
        if *a & 1 == 0 {
            *a = *a >> 1;
        } else {
            *a = (*a + MOD) >> 1;
        }
    }
    fn representation_matrix(a: &Self::Value, b: &Self::Value) -> (Self::Value, Self::Value) {
        let x = a + b;
        (
            if x >= MOD { x - MOD } else { x },
            if a < b { a + MOD - b } else { a - b },
        )
    }
}

pub trait BitOp {
    type Value: Copy;
    fn mul(a: &Self::Value, b: &Self::Value) -> Self::Value;
    fn sqrt_coef(a: &mut Self::Value);
    fn representation_matrix(a: &Self::Value, b: &Self::Value) -> (Self::Value, Self::Value);
}

pub fn bitwise_convolution<T: BitOp>(a: &[T::Value], b: &[T::Value]) -> Vec<T::Value> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    bitwise_transform::<T>(&mut a);
    bitwise_transform::<T>(&mut b);
    let mut c = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| T::mul(a, b))
        .collect::<Vec<_>>();
    bitwise_transform::<T>(&mut c);
    let k = (c.len() - 1).count_ones();
    c.iter_mut().for_each(|c| {
        for _ in 0..k {
            T::sqrt_coef(c);
        }
    });
    c
}

fn bitwise_transform<T: BitOp>(a: &mut [T::Value]) {
    let n = a.len();
    let mut k = 1;
    while k < n {
        for i in (0..n).step_by(2 * k) {
            for j in i..i + k {
                (a[j], a[j + k]) = T::representation_matrix(&a[j], &a[j + k]);
            }
        }
        k <<= 1;
    }
}
