use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        k: usize,
        queries: [(Usize1, Usize1); k],
    }

    let mut ord = mo_sort(&queries);
    let (mut l, mut r) = (0, 0);
    while 
}

const CHUNK_SIZE: usize = 512;

pub fn mo_sort(queries: &[(usize, usize)]) -> Vec<usize> {
    let n = queries.len();
    let mut query_ord = (0..n).collect::<Vec<usize>>();
    query_ord.sort_unstable_by_key(|i| {
        let (l, r) = queries[*i];
        let block = l / CHUNK_SIZE;
        (block, if block & 1 == 0 { r } else { !r })
    });
    query_ord
}
