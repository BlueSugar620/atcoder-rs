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
        .map(|x: u64| gf!((x.count_ones() as u64).min(w as u64 - x.count_ones() as u64)))
        .collect::<Vec<_>>();
    let mut h = vec![gf!(0); 1 << w];
    for &b in &b {
        h[b as usize] += gf!(1);
    }

    let ans = Bitwise::<Xor>::convolution(&g, &h);
    println!("{}", ans[..1 << w].iter().map(|x| x.value).min().unwrap());
}

pub trait Convolution {
    type Value: Copy;
    fn e() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn convolution(lhs: &[Self::Value], rhs: &[Self::Value]) -> Vec<Self::Value> {
        let size = (lhs.len() + rhs.len() - 1).next_power_of_two();
        let mut f = vec![Self::e(); size];
        let mut g = vec![Self::e(); size];
        f[..lhs.len()].copy_from_slice(lhs);
        g[..rhs.len()].copy_from_slice(rhs);

        Self::fourier_transform(&mut f);
        Self::fourier_transform(&mut g);

        let mut h = f
            .iter()
            .zip(g.iter())
            .map(|(f, g)| Self::mul(f, g))
            .collect::<Vec<_>>();

        Self::inverse_transform(&mut h);

        h.truncate(lhs.len() + rhs.len() - 1);
        h
    }
    fn fourier_transform(a: &mut [Self::Value]);
    fn inverse_transform(a: &mut [Self::Value]);
}

enum Xor {}
impl BitConv for Xor {
    fn fourier_matrix(lhs: &mut GF<998_244_353>, rhs: &mut GF<998_244_353>) {
        (*lhs, *rhs) = (*lhs + *rhs, *lhs - *rhs);
    }
    fn inverse_matrix(lhs: &mut GF<998_244_353>, rhs: &mut GF<998_244_353>) {
        (*lhs, *rhs) = (*lhs + *rhs, *lhs - *rhs);
    }
}

use std::marker::PhantomData;
pub trait BitConv {
    fn fourier_matrix(lhs: &mut GF<998_244_353>, rhs: &mut GF<998_244_353>);
    fn inverse_matrix(lhs: &mut GF<998_244_353>, rhs: &mut GF<998_244_353>);
}

struct Bitwise<T: BitConv> {
    phantom: PhantomData<T>,
}
impl<T: BitConv> Convolution for Bitwise<T> {
    type Value = GF<998_244_353>;
    fn e() -> Self::Value {
        GF::new(0)
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs * rhs
    }
    fn fourier_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in 0..b {
            let k = 1 << k;
            for a in a.chunks_exact_mut(2 * k) {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    T::fourier_matrix(x, y);
                }
            }
        }
    }
    fn inverse_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in 0..b {
            let k = 1 << k;
            for a in a.chunks_exact_mut(2 * k) {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    T::inverse_matrix(x, y);
                }
            }
        }

        let coef = GF::new(2).inv().pow(b as u64);
        for a in a.iter_mut() {
            *a *= coef;
        }
    }
}

use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GF<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> GF<MOD> {
    pub fn new(value: u64) -> Self {
        Self { value: value % MOD }
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }

    pub fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }
}

#[macro_export]
macro_rules! gf {
    ($value:expr) => {
        $crate::GF::from($value)
    };
    ($value:expr; mod $p:expr) => {
        $crate::GF::<$p>::from($value)
    };
}
macro_rules! new_from_signed {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u64> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    if x < 0 {
                        - Self::new((MOD as i64 - x as i64) as u64)
                    } else {
                        Self::new(x as u64)
                    }
                }
            }
        )*
    };
}
new_from_signed!(i8, i16, i32, i64, i128, isize);
macro_rules! new_from_unsigned {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u64> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x as u64)
                }
            }
        )*
    };
}
new_from_unsigned!(u8, u16, u32, u64, u128, usize);
impl<const MOD: u64> Debug for GF<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<const MOD: u64> Display for GF<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u64> Neg for GF<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
    }
}

impl<const MOD: u64> AddAssign<GF<MOD>> for GF<MOD> {
    fn add_assign(&mut self, rhs: GF<MOD>) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}
impl<const MOD: u64> SubAssign<GF<MOD>> for GF<MOD> {
    fn sub_assign(&mut self, rhs: GF<MOD>) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}
impl<const MOD: u64> MulAssign<GF<MOD>> for GF<MOD> {
    fn mul_assign(&mut self, rhs: GF<MOD>) {
        self.value *= rhs.value;
        self.value %= MOD;
    }
}
impl<const MOD: u64> DivAssign<GF<MOD>> for GF<MOD> {
    fn div_assign(&mut self, rhs: GF<MOD>) {
        self.value *= rhs.inv().value;
        self.value %= MOD;
    }
}
macro_rules! gf_ops {
    ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
    )*) => {$(
        impl<const MOD: u64> $trait_assign<&GF<MOD>> for GF<MOD> {
            fn $fn_assign(&mut self, rhs: &GF<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u64, T: Into<GF<MOD>>> $trait<T> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u64> $trait<&GF<MOD>> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u64, T: Into<GF<MOD>>> $trait<T> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u64> $trait<&GF<MOD>> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}
gf_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}

impl<const MOD: u64> Sum for GF<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, a| acc + a)
    }
}
impl<'a, const MOD: u64> Sum<&'a Self> for GF<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u64> Product for GF<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, a| acc * a)
    }
}
impl<'a, const MOD: u64> Product<&'a Self> for GF<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
