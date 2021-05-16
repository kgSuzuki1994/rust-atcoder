use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    if n == m {
        println!("Yes");
    } else {
        println!("No");
    }
}
