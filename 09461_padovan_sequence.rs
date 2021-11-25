use std::io;

fn main(){
    let mut dp: [i64; 101] = [0; 101];
    dp[1] = 1;
    dp[2] = 1;
    dp[3] = 1;
    dp[4] = 2;
    for i in 5..101{
        dp[i] = dp[i - 1] + dp[i - 5];
    }

    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Fail");
    let t: i32 = t.trim().parse().expect("Fail");

    for i in 0..t{
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Fail");
        let n: usize = n.trim().parse().expect("Fail");
        println!("{}", dp[n]);
    }
}