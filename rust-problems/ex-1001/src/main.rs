use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("fudeu 1");
    let n1: i32 = n1.trim().parse().unwrap();
    io::stdin()
        .read_line(&mut n2)
        .expect("fudeu 2");
    
    let n2: i32 = n2.trim().parse().unwrap();


    println!("X = {}\n", n1 + n2);
}