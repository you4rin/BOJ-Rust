use std::io;

fn main(){
    const MOD: i32 = 10007;
    let mut dp: [i32; 1001] = [0; 1001];
    dp[0] = 1;
    dp[1] = 1;
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: usize = n.trim().parse().expect("Fail");
    for i in 2..=n{
        dp[i] = dp[i - 1] + dp[i - 2];
        dp[i] %= MOD;
    }
    println!("{}", dp[n]);
}