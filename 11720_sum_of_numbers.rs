use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: usize = n.trim().parse().expect("Fail");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Fail");
    let s = s.trim().as_bytes();

    let mut ans = 0;
    
    for i in 0..n{
        ans += (s[i] as u32) - ('0' as u32);
    }

    println!("{}", ans);
}