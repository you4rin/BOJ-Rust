use std::io;
use std::cmp;

fn main(){
    const INF: i32 = 1000000000;
    let mut burger = INF;
    let mut coke = INF;
    for i in 0..3{
        let mut price = String::new();
        io::stdin()
            .read_line(&mut price)
            .expect("Fail");
        let price = price.trim().parse().expect("Fail");
        burger = cmp::min(burger, price);
    }
    for i in 0..2{
        let mut price = String::new();
        io::stdin()
            .read_line(&mut price)
            .expect("Fail");
        let price = price.trim().parse().expect("Fail");
        coke = cmp::min(coke, price);
    }
    println!("{}", burger + coke - 50);
}