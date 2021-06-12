use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut result = 0;
    for i in 0..n {
        result += a[i] * b[i];
    }
    if result == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
