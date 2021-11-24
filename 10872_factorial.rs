use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");
    
    let mut ans = 1;
    for i in 1..=n{
        ans *= i;
    }
    println!("{}", ans);
}