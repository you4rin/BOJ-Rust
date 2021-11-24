use std::io;

fn main(){
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Fail");
    let c: u8 = c.as_bytes()[0];
    println!("{}", c);
}