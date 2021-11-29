use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n) 
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");
    let mut cnt = 0;
    let mut cur = 0;

    loop{
        cur += 1;
        let mut chk = cur;
        let mut six = 0;
        let mut flag = false;
        while chk > 0{
            if chk % 10 == 6{
                six += 1;
                if six == 3{
                    flag =true;
                    break;
                }
            }
            else{
                six = 0;
            }
            chk /= 10;
        }
        if flag{
            cnt += 1;
        }
        if cnt == n{
            break;
        }
    }
    println!("{}", cur);
}