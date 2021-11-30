use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n) 
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    let mut flag = false;

    for i in 1..n{
        let mut tot = i;
        let mut cur = i;
        while cur != 0{
            tot += cur % 10;
            cur /= 10;
        }
        if tot == n{
            println!("{}", i);
            flag = true;
            break;
        }
    }
    if !flag{
        println!("0");
    }
}