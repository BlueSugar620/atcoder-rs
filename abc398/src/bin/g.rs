use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let lca = LCA::new(&uv);
    let mut edges = std::collections::BTreeSet::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            let x = lca.dist(i, j);
            if x > 1 && (x & 1) == 1 {
                edges.insert((i, j));
            }
        }
    }

    if edges.len() & 1 == 1 {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}

pub struct LCA {
    idx: Vec<usize>,
    sparse_table: Vec<Vec<(usize, usize)>>,
    dist: Vec<usize>,
}
impl LCA {
    pub fn new(e: &[(usize, usize)]) -> Self {
        let n = e.len() + 1;
        let _e = e;
        let mut e = vec![vec![]; n];
        for &(u, v) in _e {
            e[u].push(v);
            e[v].push(u);
        }
        let mut dist = vec![!0; n];
        let mut eular_tour = Vec::with_capacity(2 * n - 1);
        Self::dfs(0, 0, &e, &mut eular_tour, &mut dist);
        let mut idx = vec![!0; n];
        for (i, &(_, u)) in eular_tour.iter().enumerate() {
            if idx[u] == !0 {
                idx[u] = i;
            }
        }
        let mut sparse_table = vec![eular_tour];
        let mut i = 1;
        while i < n {
            let prev = sparse_table.last().unwrap();
            let crnt = prev
                .iter()
                .zip(&prev[i..])
                .map(|(x, y)| *x.min(y))
                .collect::<Vec<_>>();
            sparse_table.push(crnt);
            i <<= 1;
        }
        Self {
            idx,
            sparse_table,
            dist,
        }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        let mut pu = self.idx[u];
        let mut pv = self.idx[v];
        if pu > pv {
            std::mem::swap(&mut pu, &mut pv);
        }
        let x = (pv + 1 - pu).ilog2() as usize;
        self.sparse_table[x][pu]
            .min(self.sparse_table[x][pv + 1 - (1 << x)])
            .1
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        let mut pu = self.idx[u];
        let mut pv = self.idx[v];
        if pu > pv {
            std::mem::swap(&mut pu, &mut pv);
        }
        let x = (pv + 1 - pu).ilog2() as usize;
        let lca_dist = self.sparse_table[x][pu]
            .min(self.sparse_table[x][pv + 1 - (1 << x)])
            .0;
        self.dist[u] + self.dist[v] - 2 * lca_dist
    }

    fn dfs(
        u: usize,
        d: usize,
        e: &[Vec<usize>],
        eular_tour: &mut Vec<(usize, usize)>,
        dist: &mut [usize],
    ) {
        dist[u] = d;
        for &v in &e[u] {
            if dist[v] == !0 {
                eular_tour.push((d, u));
                Self::dfs(v, d + 1, e, eular_tour, dist);
            }
        }
        eular_tour.push((d, u));
    }
}
