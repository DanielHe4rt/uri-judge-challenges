use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    io::stdin().read_line(&mut n1).expect("fodase");
    io::stdin().read_line(&mut n2).expect("fodase");

    let n1: f32 = n1.trim().parse().expect("");
    let n2: f32 = n2.trim().parse().expect("");
    let result: f32 = ((n1 * 3.5) + (n2 * 7.5)) / 11.0;
    println!("MEDIA = {:.1$}",result, 5);
}
