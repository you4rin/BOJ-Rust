use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    for i in 0..n{
        let end = n - i;
        for j in 0..i{
            print!(" ");
        }
        for j in 0..end{
            print!("*");
        }
        println!();
    }
}