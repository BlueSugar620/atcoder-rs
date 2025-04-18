use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let sri = StaticRangeInversions::new(&p, 1 << 20);
    let tot = sri.calc_inversions(..);

    let mut ans = GF::<MOD>::new(0);
    for i in 0..=n - k {
        let x = sri.calc_inversions(i..i + k);
        ans += (gf!(tot) - gf!(x) + gf!(k * (k - 1)) / gf!(4)) * gf!(n - k + 1).inv();
    }

    println!("{}", ans);
}

use std::{
    cmp::Reverse,
    ops::{Bound, RangeBounds},
};

const CHUNK_SIZE: usize = 512;

pub struct StaticRangeInversions<T: Copy + PartialOrd> {
    n: usize,
    from_left: Vec<Vec<usize>>,
    from_right: Vec<Vec<usize>>,
    sorted_chunks: Vec<Vec<(usize, T)>>,
}

impl<T: Copy + PartialOrd> StaticRangeInversions<T> {
    pub fn new(a: &[T], inf: T) -> Self {
        let n = a.len();
        let mut a = a.to_vec();
        while a.len() % CHUNK_SIZE > 0 {
            a.push(inf);
        }

        let from_left = Self::create_block(&a);
        let mut from_right =
            Self::create_block(&a.iter().rev().map(|a| Reverse(*a)).collect::<Vec<_>>());
        from_right.reverse();

        let sorted_chunks = a
            .chunks(CHUNK_SIZE)
            .map(|a| {
                let mut a = a.iter().copied().enumerate().collect::<Vec<_>>();
                a.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                a
            })
            .collect::<Vec<_>>();

        Self {
            n,
            from_left,
            from_right,
            sorted_chunks,
        }
    }

    pub fn calc_inversions<R: RangeBounds<usize>>(&self, range: R) -> usize {
        let (l, r) = unzip(range, self.n);
        let ll = l - l % CHUNK_SIZE;
        let rr = r + (CHUNK_SIZE - r % CHUNK_SIZE) % CHUNK_SIZE;
        self.from_left[ll / CHUNK_SIZE][r - ll]
            + self.from_right[rr / CHUNK_SIZE - 1][rr - l]
            + self.calc_ends(
                l % CHUNK_SIZE,
                r % CHUNK_SIZE,
                ll / CHUNK_SIZE,
                rr / CHUNK_SIZE,
            )
            - self.from_left[ll / CHUNK_SIZE][rr - ll]
    }

    fn create_block<S: Copy + PartialOrd>(a: &[S]) -> Vec<Vec<usize>> {
        let mut res: Vec<Vec<usize>> = vec![];
        let mut sorted_block = vec![];
        for (chunk_idx, chunk) in a.chunks(CHUNK_SIZE).enumerate().rev() {
            let sorted_chunk = {
                let mut sorted_chunk = (0..CHUNK_SIZE).collect::<Vec<_>>();
                sorted_chunk.sort_by(|i, j| chunk[*j].partial_cmp(&chunk[*i]).unwrap());
                sorted_chunk
            };
            let c = {
                let mut c = vec![0; sorted_block.len()];
                let mut idx = 0;
                for &k in sorted_block.iter() {
                    while idx < CHUNK_SIZE
                        && chunk[sorted_chunk[idx]] > a[CHUNK_SIZE * (chunk_idx + 1) + k]
                    {
                        idx += 1;
                    }
                    c[k] = idx;
                }
                c
            };
            sorted_block = Self::merge_sort(
                &a[CHUNK_SIZE * (chunk_idx + 1)..],
                &chunk,
                &sorted_block,
                &sorted_chunk,
            );
            let mut res_i = vec![0];
            for (j, aj) in chunk.iter().enumerate() {
                res_i.push(res_i.last().unwrap() + chunk[..j].iter().filter(|al| *al > aj).count());
            }
            for (k, c) in c.iter().enumerate() {
                res_i.push(
                    res_i.last().unwrap() + res.last().unwrap()[k + 1] + c - res.last().unwrap()[k],
                );
            }
            res.push(res_i);
        }
        res.reverse();
        res
    }

    fn merge_sort<S: Copy + PartialOrd>(a: &[S], b: &[S], x: &[usize], y: &[usize]) -> Vec<usize> {
        let mut res = vec![0; x.len() + y.len()];
        let mut x_idx = 0;
        let mut y_idx = 0;
        while x_idx < x.len() && y_idx < y.len() {
            if a[x[x_idx]] >= b[y[y_idx]] {
                res[x_idx + y_idx] = x[x_idx] + CHUNK_SIZE;
                x_idx += 1;
            } else {
                res[x_idx + y_idx] = y[y_idx];
                y_idx += 1;
            }
        }
        if x_idx < x.len() {
            res[x_idx + y_idx..].clone_from_slice(
                &x[x_idx..]
                    .iter()
                    .map(|x| x + CHUNK_SIZE)
                    .collect::<Vec<_>>(),
            );
        } else if y_idx < y.len() {
            res[x_idx + y_idx..].clone_from_slice(&y[y_idx..]);
        }
        res
    }

    fn calc_ends(&self, l: usize, r: usize, ll: usize, rr: usize) -> usize {
        if r == 0 {
            return 0;
        }
        let mut res = 0;
        let mut l_idx = 0;
        let mut cnt = 0;
        for &(r_idx, r_val) in self.sorted_chunks[rr - 1].iter() {
            while l_idx < CHUNK_SIZE && self.sorted_chunks[ll][l_idx].1 > r_val {
                if l > self.sorted_chunks[ll][l_idx].0 {
                    cnt += 1;
                }
                l_idx += 1;
            }
            if r_idx >= r {
                res += cnt;
            }
        }
        res
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
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
