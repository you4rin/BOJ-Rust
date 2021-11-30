use std::io::{self, Write};

fn main(){
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = Vec::new();
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    for i in 0..n{
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Fail");
        let x: i32 = x.trim().parse().expect("Fail");
        arr.push(x);
    }

    arr.sort();

    for i in arr{
        writeln!(out, "{}", i);
    }
}