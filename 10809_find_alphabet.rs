use std::io;

fn main(){
    let mut arr: [i32; 26] = [-1; 26];
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Fail");
    let s: String = s.trim().to_string();
    let len = s.len();
    for i in 0..len{
        let idx: usize = (s.as_bytes()[i] - ('a' as u8)) as usize;
        if arr[idx] == -1{
            arr[idx] = i as i32;
        }
    }
    for (i, elem) in arr.iter().enumerate(){
        print!("{} ",elem);
    }
}