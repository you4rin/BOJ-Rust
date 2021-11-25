use std::io;

fn main(){
    let mut zero: [i64; 91] = [0; 91];
    let mut one: [i64; 91] = [0; 91];
    one[1] = 1;
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: usize = n.trim().parse().expect("Fail");
    for i in 2..=n{
        one[i] = zero[i - 1];
        zero[i] = one[i - 1] + zero[i - 1];
    }
    println!("{}", zero[n] + one[n]);
}