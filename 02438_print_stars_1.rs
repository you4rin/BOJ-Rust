use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    for i in 0..n{
        for j in 0..=i{
            print!("*");
        }
        println!();
    }
}