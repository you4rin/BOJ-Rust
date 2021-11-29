use std::io;

fn main(){
    loop{
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Fail");
        let s = s.trim().as_bytes();
        
        if (s[0] as i32) == ('0' as i32){
            break;
        }
        
        let len = s.len();
        let mut st: usize = 0;
        let mut ed: usize = len - 1;
        let mut flag = false;
        while st < ed{
            if s[st] != s[ed]{
                flag = true;
                break;
            }
            st += 1;
            ed -= 1;
        }
        if flag{
            println!("no");
        }
        else{
            println!("yes");
        }
    }
}