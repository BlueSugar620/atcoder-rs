use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
    }

    let mut diffs = vec![];
    let mut idx = 0;
    for (i, &ai) in a.iter().enumerate() {
        while idx < n && a[idx] < 2 * ai {
            idx += 1;
        }
        diffs.push((1, idx - i));
    }

    let segtree = SegmentTree::<O>::new(&diffs);

    for _ in 0..q {
        input! {
            l: Usize1,
            r: usize,
        }
        let ans = segtree.max_right(l, |(d, x)| l + d - 1 + (*x).max(d - 1) <= r);
        println!("{}", ans - l);
    }
}

enum O {}
impl Monoid for O {
    type Value = (usize, usize);
    fn e() -> Self::Value {
        (1, 0)
    }
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        (lhs.0 + rhs.0, lhs.1.max(rhs.1))
    }
}

pub trait Monoid {
    type Value: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

use std::ops::RangeBounds;
pub struct SegmentTree<T: Monoid> {
    values: Vec<T::Value>,
    len: usize,
}
impl<T: Monoid> SegmentTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len().next_power_of_two();
        let mut values = vec![T::e(); 2 * n];
        values[n..n + a.len()].clone_from_slice(a);
        for i in (1..n).rev() {
            values[i] = T::op(&values[2 * i], &values[2 * i + 1]);
        }
        Self {
            values,
            len: a.len(),
        }
    }

    pub fn get_at(&self, idx: usize) -> T::Value {
        self.values[idx + self.values.len() / 2]
    }

    pub fn update_at(&mut self, i: usize, x: T::Value) {
        let mut i = i + self.values.len() / 2;
        self.values[i] = x;
        i >>= 1;
        while i > 0 {
            self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
            i >>= 1;
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let (l, r) = unzip(range, self.values.len() / 2);
        let (mut l, mut r) = (l + self.values.len() / 2, r + self.values.len() / 2);
        let mut left = T::e();
        let mut right = T::e();
        while l < r {
            if l % 2 == 1 {
                left = T::op(&left, &self.values[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = T::op(&self.values[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
    }

    pub fn max_right<P: Fn(&T::Value) -> bool>(&self, l: usize, f: P) -> usize {
        let n = self.values.len() / 2;
        if l == n {
            return self.len;
        }
        let mut l = l + n;
        let mut r = 2 * n;
        let mut x = T::e();
        while l < r {
            if l & 1 == 1 {
                let y = T::op(&x, &self.values[l]);
                if !f(&y) {
                    while l < n {
                        l *= 2;
                        let z = T::op(&x, &self.values[l]);
                        if f(&z) {
                            x = z;
                            l += 1;
                        }
                    }
                    return l - n;
                } else {
                    x = y;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.len
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    use std::ops::Bound;
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
