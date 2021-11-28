use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");
    let mut cur = n;
    let mut cnt = 0;
    loop{
        cur = (cur % 10) * 10 + (cur / 10 + cur % 10) % 10;
        cnt += 1;
        if cur == n{
            break;
        }
    }
    println!("{}", cnt);
}