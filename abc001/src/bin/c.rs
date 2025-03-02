use proconio::input;

const DIR: [&str; 16] = [
    "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW",
    "NNW",
];

fn main() {
    input! {
        deg: u16,
        dis: u32,
    }

    let dir = DIR[((deg + 112) % 3600 / 225) as usize];
    let border = [2, 15, 33, 54, 79, 107, 138, 171, 207, 244, 284, 326];
    let w = border
        .iter()
        .enumerate()
        .find(|(_i, b)| dis < *b * 6 + 3)
        .map(|x| x.0)
        .unwrap_or(12);

    println!("{} {}", if w == 0 { "C" } else { dir }, w);
}
