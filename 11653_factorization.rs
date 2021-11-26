use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let mut n: i64 = n.trim().parse().expect("Fail");

    for i in 2..n{
        if i * i > n{
            break;
        }
        while n % i == 0{
            println!("{}", i);
            n /= i;
        }
    }
    if n != 1{
        println!("{}", n);
    }
}