use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: usize = n.trim().parse().expect("Fail");

    if n >= 90{
        println!("A");
    }
    else if n >= 80{
        println!("B");
    }
    else if n >= 70{
        println!("C");
    }
    else if n >= 60{
        println!("D");
    }
    else{
        println!("F");
    }
}