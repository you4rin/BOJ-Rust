use std::io;

fn main(){
    let arr: [i32; 11] = [0, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274];

    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Fail");
    let t: i32 = t.trim().parse().expect("Fail");

    for i in 0..t{
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Fail");
        let n: usize = n.trim().parse().expect("Fail");
        println!("{}", arr[n]);
    }
}