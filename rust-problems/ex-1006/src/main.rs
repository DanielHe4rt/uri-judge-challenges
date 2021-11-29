use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut n3 = String::new();

    io::stdin().read_line(&mut n1).expect("");
    io::stdin().read_line(&mut n2).expect("");
    io::stdin().read_line(&mut n3).expect("");

    let n1: f32 = n1.trim().parse().expect("");
    let n2: f32 = n2.trim().parse().expect("");
    let n3: f32 = n3.trim().parse().expect("");
    let result: f32 = ((n1 * 2.0) + (n2 * 3.0) + (n3 * 5.0)) / 10.0;

    println!("MEDIA = {:.1$}", result, 1);
}
