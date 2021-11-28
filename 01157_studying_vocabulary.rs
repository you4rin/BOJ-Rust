use std::io;

fn main(){
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Fail");
    let s = s.trim().as_bytes();
    let len = s.len();

    let mut arr: [i32; 26] = [0; 26];

    for i in 0..len{
        if (s[i] as usize) >= ('a' as usize){
            arr[(s[i] as usize) - ('a' as usize)] += 1;
        }
        else{
            arr[(s[i] as usize) - ('A' as usize)] += 1;
        }
    }

    let mut mx = -1;
    let mut midx: u8 = 0;
    let mut mcnt = 1;
    for i in 0..26{
        if arr[i] > mx{
            mx = arr[i];
            midx = i as u8;
            mcnt = 1;
        }
        else if arr[i] == mx{
            midx = 0;
            mcnt += 1;
        }
    }

    if mcnt != 1{
        println!("?");
    }
    else{
        println!("{}", (midx + ('A' as u8)) as char);
    }
}