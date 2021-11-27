use std::io;
use std::io::Write;
// use std::io::{self, Write};

fn main(){
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    for i in 1..=n{
        writeln!(out, "{}", i); // cannot use println!
    }
}