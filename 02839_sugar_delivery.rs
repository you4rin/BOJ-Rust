use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n) 
        .expect("Fail");
    let n: i32 = n.trim().parse().expect("Fail");
    let mut three = 0;
    let mut five = n / 5;

    loop{
        if three * 3 + five * 5 == n{
            break;
        }
        if three * 3 + five * 5 > n{
            if five == 0{
                break;
            }
            five -= 1;
        }
        else{
            three += 1;
        }
    }

    if three * 3 + five * 5 == n{
        println!("{}", three + five);
    }
    else{
        println!("-1");
    }
}