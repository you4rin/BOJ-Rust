use std::io;

fn main(){
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Fail");
    let x: i32 = x.trim().parse().expect("Fail");

    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Fail");
    let y: i32 = y.trim().parse().expect("Fail");

    if x > 0 && y > 0{
        println!("1");
    }
    else if x < 0 && y > 0{
        println!("2");
    }
    else if x < 0 && y < 0{
        println!("3");
    }
    else{
        println!("4");
    }
}