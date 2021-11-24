use std::io;

fn main(){
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Fail");
    let a: i32 = a.trim().parse().expect("Fail");

    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Fail");
    let b: i32 = b.trim().parse().expect("Fail");
    println!("{}", (b % 10) * a);
    println!("{}", (b / 10 % 10) * a);
    println!("{}", (b / 100) * a);
    println!("{}", a * b);
}