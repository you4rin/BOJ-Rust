use std::io;

fn main(){
    let n = 9;
    let mut mx = 0;
    let mut midx = 0;
    for i in 0..n{
        let mut cur = String::new();
        io::stdin()
            .read_line(&mut cur)
            .expect("Fail");
        let cur: i32 = cur.trim().parse().expect("Fail");

        if cur > mx{
            mx = cur;
            midx = i + 1;
        }
    }
    println!("{}", mx);
    println!("{}", midx);
}