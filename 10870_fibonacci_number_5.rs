use std::io;

fn main(){
    let mut dp: [i32; 21] = [0; 21];
    dp[1] = 1;
    for i in 2..21{
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: usize = n.trim().parse().expect("Fail");
    println!("{}", dp[n]);
}