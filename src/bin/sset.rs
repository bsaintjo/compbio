use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let n = stdin.lines().next().unwrap().unwrap();
    let n = u32::from_str(&n).unwrap();
    let mut result = 2;
    for _ in 1 .. n {
        result = (result * 2) % 1_000_000;
    }
    println!("{result}");
}
