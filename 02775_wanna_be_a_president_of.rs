use std::io;

fn main(){
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for i in 0..15{
        arr.push(vec![0; 15]);
    }
    for i in 1..15{
        arr[0][i] = i as i32;
    }
    for i in 1..15{
        for j in 0..15{
            for k in 0..=j{
                arr[i][j] += arr[i-1][k];
            }
        }
    }

    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Fail");
    let t: i32 = t.trim().parse().expect("Fail");

    for i in 0..t{
        let mut k = String::new();
        io::stdin()
            .read_line(&mut k)
            .expect("Fail");
        let k: usize = k.trim().parse().expect("Fail");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Fail");
        let n: usize = n.trim().parse().expect("Fail");
        println!("{}", arr[k][n]);
    }
}