use std::io::{self, Write};

fn main(){
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    let mut v: Vec<i32> = vec![0; 10001];

    for i in 0..n{
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Fail");
        let x: usize = x.trim().parse().expect("Fail");
        v[x] += 1;
    }

    let mut end = 0;

    for i in 1..10001{
        v[i] += v[i - 1];
    }

    for i in 0..10001{
        while end < v[i]{
            writeln!(out, "{}", i);
            end += 1;
        }
    }
}