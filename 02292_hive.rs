use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i64 = n.trim().parse().expect("Fail");
    let mut cnt: i64 = 1;
    loop{
        if 3 * cnt * (cnt - 1) + 1 >= n{
            println!("{}", cnt);
            break;
        }
        cnt += 1;
    }
}