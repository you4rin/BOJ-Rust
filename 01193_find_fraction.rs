use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let mut n: i32 = n.trim().parse().expect("Fail");

    let mut tot = 1;

    loop{
        if n <= tot{
            if tot % 2 == 1{
                println!("{}/{}", tot + 1 - n, n);
            }
            else{
                println!("{}/{}", n, tot + 1 - n);
            }
            break;
        }
        n -= tot;
        tot += 1;
    }
}