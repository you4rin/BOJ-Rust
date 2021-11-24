use std::io;

fn main(){
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Fail");
    let a: i32 = a.trim().parse().expect("Fail");

    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Fail");
    let b: i32 = b.trim().parse().expect("Fail");

    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Fail");
    let c: i32 = c.trim().parse().expect("Fail");

    let mut tot = a * b * c;
    let mut cnt = [0; 10];
    while tot != 0{
        let idx: usize = (tot % 10) as usize;
        cnt[idx] += 1;
        tot /= 10;
    }
    for (i, elem) in cnt.iter().enumerate(){
        println!("{}", elem);
    }
}