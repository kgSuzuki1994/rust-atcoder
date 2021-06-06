use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }
    if x == y {
        println!("{}", x);
    } else if x != 0 && y != 0 {
        println!("0");
    } else if x != 1 && y != 1 {
        println!("1");
    } else {
        println!("2");
    }
}
