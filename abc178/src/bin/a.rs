use proconio::input;
use std::mem;

fn main() {
    input! {
        x: i64,
    }
    if x == 0 {
        println!("1");
    } else {
        println!("0")
    }
}
