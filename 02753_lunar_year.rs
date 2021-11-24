use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");
    if n % 400 == 0{
        println!("1");
    }
    else if n % 100 == 0{
        println!("0");
    }
    else if n % 4 == 0{
        println!("1");
    }
    else{
        println!("0");
    }
}