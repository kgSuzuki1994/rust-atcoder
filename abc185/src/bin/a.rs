use proconio::input;

fn main() {
    input! {
        vec: [i32; 4],
    }

    let mut ans = 100;
    for &n in &vec {
        if n < ans {
            ans = n;
        }
    }
    println!("{}", ans);
}
