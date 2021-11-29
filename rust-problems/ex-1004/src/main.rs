use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    io::stdin().read_line(&mut n1).expect("");
    io::stdin().read_line(&mut n2).expect("");

    let n1: i32 = n1.trim().parse().expect("");
    let n2: i32 = n2.trim().parse().expect("");

    println!("PROD = {}", n1 * n2);
}
