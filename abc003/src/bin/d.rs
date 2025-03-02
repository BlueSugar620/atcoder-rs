use proconio::input;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        r: u64,
        c: u64,
        x: u64,
        y: u64,
        d: u64,
        l: u64,
    }

    let combmod = CombMod::new(x * y);

    let ans = (0..1 << 4)
        .map(|b| {
            let dx = ((b >> 0) & 1) + ((b >> 1) & 1);
            let dy = ((b >> 2) & 1) + ((b >> 3) & 1);
            if dx > x || dy > y {
                return gf!(0);
            }
            let s = (x - dx) * (y - dy);
            if s < d + l {
                return gf!(0);
            }
            combmod.comb(s, d)
                * combmod.comb(s - d, l)
                * if (dx + dy) % 2 == 0 { gf!(1) } else { -gf!(1) }
        })
        .sum::<GF<MOD>>()
        * gf!(r - x + 1)
        * gf!(c - y + 1);

    println!("{}", ans);
}

pub struct CombMod {
    factorial: Vec<GF<MOD>>,
}

impl CombMod {
    pub fn new(n: u64) -> Self {
        let mut factorial = vec![GF::<MOD>::new(1)];
        for i in 1..=n {
            factorial.push(factorial.last().unwrap() * gf!(i));
        }
        Self { factorial }
    }

    pub fn comb(&self, n: u64, r: u64) -> GF<MOD> {
        self.factorial[n as usize] / self.factorial[r as usize] / self.factorial[(n - r) as usize]
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
