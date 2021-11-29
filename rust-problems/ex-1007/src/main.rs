use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut n3 = String::new();
    let mut n4 = String::new();

    io::stdin().read_line(&mut n1).expect("");
    io::stdin().read_line(&mut n2).expect("");
    io::stdin().read_line(&mut n3).expect("");
    io::stdin().read_line(&mut n4).expect("");

    let n1: f32 = n1.trim().parse().expect("");
    let n2: f32 = n2.trim().parse().expect("");
    let n3: f32 = n3.trim().parse().expect("");
    let n4: f32 = n4.trim().parse().expect("");
    let result: f32 = n1 * n2 - n3 * n4;

    println!("DIFERENCA = {:.1$}", result, 0);
}
