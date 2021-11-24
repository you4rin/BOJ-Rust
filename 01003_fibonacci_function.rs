use std::io;

fn main(){
    let mut zero: [i64; 41] = [0; 41];
    let mut one: [i64; 41] = [0; 41];
    zero[0] = 1;
    one[1] = 1;
    for i in 2..41{
        zero[i] = zero[i - 1] + zero[i - 2];
        one[i] = one[i - 1] + one[i - 2];
    }

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
        let n: i32 = n.trim().parse().expect("Fail");
        let n: usize = n as usize;
        println!("{} {}", zero[n], one[n]);
    }
}