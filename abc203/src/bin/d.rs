use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[u32; n]; n],
    }
    let mut l = 0;
    let mut r = 1 << 30;
    while r - l > 1 {
        let o = (r + l) / 2;
        let inclusive_scan = InclusiveScan2d::<O>::new(
            &a.iter()
                .map(|a| {
                    a.iter()
                        .map(|a| if *a >= o { 1 } else { 0 })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );
        let flag = (0..=n - k)
            .all(|i| (0..=n - k).all(|j| inclusive_scan.area(i..i + k, j..j + k) > k * k / 2));
        if flag {
            l = o;
        } else {
            r = o;
        }
    }
    println!("{}", l);
}

enum O {}
impl InclusiveScan2dOp for O {
    type Value = usize;
    fn e() -> Self::Value {
        0
    }
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs.wrapping_add(*rhs)
    }
    fn inv(val: &Self::Value) -> Self::Value {
        val.wrapping_neg()
    }
}

use std::ops::{Bound, RangeBounds};

pub trait InclusiveScan2dOp {
    type Value: Copy;
    fn e() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct InclusiveScan2d<T: InclusiveScan2dOp> {
    value: Vec<Vec<T::Value>>,
}

impl<T: InclusiveScan2dOp> InclusiveScan2d<T> {
    pub fn new(a: &Vec<Vec<T::Value>>) -> Self {
        let h = a.len();
        let w = a[0].len();
        let mut value = vec![vec![T::e(); w + 1]; h + 1];
        for (i, a) in a.iter().enumerate() {
            for (j, a) in a.iter().enumerate() {
                value[i + 1][j + 1] = T::add(&value[i + 1][j], a);
            }
        }
        for j in 0..w {
            for i in 0..h {
                value[i + 1][j + 1] = T::add(&value[i + 1][j + 1], &value[i][j + 1]);
            }
        }
        Self { value }
    }

    pub fn area<R: RangeBounds<usize>>(&self, x: R, y: R) -> T::Value {
        let h = self.value.len();
        let w = self.value[0].len();
        let (xl, xr) = unzip(x, h);
        let (yl, yr) = unzip(y, w);
        T::add(
            &T::add(
                &T::add(&self.value[xr][yr], &self.value[xl][yl]),
                &T::inv(&self.value[xr][yl]),
            ),
            &T::inv(&self.value[xl][yr]),
        )
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
