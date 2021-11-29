use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    io::stdin()
        .read_line(&mut n1)
        .expect("fodase");

    io::stdin()
        .read_line(&mut n2)
        .expect("fodase");

    let n1: i32 = n1.trim().parse().expect("minha vida é bela");
    let n2: i32 = n2.trim().parse().expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    

    println!("SOMA = {}", n1 + n2);
}
