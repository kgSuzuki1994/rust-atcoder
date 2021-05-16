use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!("{}", 2 * a + 100 - b);
}
