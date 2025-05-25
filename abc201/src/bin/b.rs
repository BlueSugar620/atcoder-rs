use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, u32); n],
    }
    st.sort_unstable_by_key(|x| !x.1);
    println!("{}", st[1].0);
}
