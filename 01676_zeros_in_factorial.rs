use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n) 
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    let mut ans = 0;

    for i in 1..=n{
        let mut cur = i;
        while cur % 5 == 0{
            cur /= 5;
            ans += 1;
        }
    }

    println!("{}", ans);
}