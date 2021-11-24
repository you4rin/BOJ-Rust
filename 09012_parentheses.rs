use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");

    for i in 0..n{
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Fail");
        let s = s.trim().to_string();
        
        let mut credit: i32 = 0;
        let mut flag: bool = false;
        for c in s.chars(){
            if c == '('{
                credit += 1;
            }
            else{
                credit -= 1;
                if credit < 0{
                    flag = true;
                    break;
                }
            }
        }
        if credit != 0{
            flag = true;
        }
        if flag{
            println!("NO");
        }
        else{
            println!("YES");
        }
    }
}