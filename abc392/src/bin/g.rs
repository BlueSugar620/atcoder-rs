use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let mut idx = vec![gf!(0); 1_000_000 + 1];
    for &s in &s {
        idx[s] = gf!(1);
    }

    let conv = NumberTheoric998244353::convolution(&idx, &idx);

    let mut ans = 0;
    for &s in &s {
        ans += (conv[2 * s].value - 1) / 2;
    }

    println!("{}", ans);
}

const SUM_E: [u64; 22] = [
    911660635, 509520358, 369330050, 332049552, 983190778, 123842337, 238493703, 975955924,
    603855026, 856644456, 131300601, 842657263, 730768835, 942482514, 806263778, 151565301,
    510815449, 503497456, 743006876, 741047443, 56250497, 867605899,
];
const SUM_IE: [u64; 22] = [
    86583718, 372528824, 373294451, 645684063, 112220581, 692852209, 155456985, 797128860,
    90816748, 860285882, 927414960, 354738543, 109331171, 293255632, 535113200, 308540755,
    121186627, 608385704, 438932459, 359477183, 824071951, 103369235,
];

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

        assert_eq!(f.len(), size);
        assert_eq!(g.len(), size);

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

enum NumberTheoric998244353 {}
impl Convolution for NumberTheoric998244353 {
    type Value = GF<998_244_353>;
    fn e() -> Self::Value {
        gf!(0)
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs * rhs
    }
    fn fourier_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let mut k = n;
        while k > 1 {
            k >>= 1;
            let mut now = gf!(1);
            for i in (0..n).step_by(2 * k) {
                for j in i..i + k {
                    (a[j], a[j + k]) = (a[j] + a[j + k] * now, a[j] - a[j + k] * now);
                }
                now *= gf!(SUM_E[(!(i / 2 / k)).trailing_zeros() as usize]);
            }
        }
    }
    fn inverse_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let mut k = 1;
        while k < n {
            let mut now = gf!(1);
            for i in (0..n).step_by(2 * k) {
                for j in i..i + k {
                    (a[j], a[j + k]) = (a[j] + a[j + k], (a[j] - a[j + k]) * now);
                }
                now *= gf!(SUM_IE[(!(i / 2 / k)).trailing_zeros() as usize]);
            }
            k <<= 1;
        }
        let ik = gf!(2).inv().pow(n.trailing_zeros() as u64);
        for a in a.iter_mut() {
            *a *= ik;
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
