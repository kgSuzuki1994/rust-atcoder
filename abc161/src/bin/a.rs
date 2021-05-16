use proconio::input;
use std::mem;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }
    let mut a = x;
    let mut b = y;
    let mut c = z;
    mem::swap(&mut a, &mut b);
    mem::swap(&mut a, &mut c);

    println!("{} {} {}", a, b, c);
}
