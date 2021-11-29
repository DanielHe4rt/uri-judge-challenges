use std::io;

fn main() {
    let mut p1_in = String::new();
    let mut p2_in = String::new();

    io::stdin().read_line(&mut p1_in).expect("");
    io::stdin().read_line(&mut p2_in).expect("");

    let mut p1_in = p1_in.split_whitespace();
    let mut p2_in = p2_in.split_whitespace();
    
    let x1: f64 = p1_in.next().expect("").trim().parse().expect("");
    let y1: f64 = p1_in.next().expect("").trim().parse().expect("");
    

    let x2: f64 = p2_in.next().expect("").trim().parse().expect("");
    let y2: f64 = p2_in.next().expect("").trim().parse().expect("");

    let result: f64 = ((x2 - x1 as f64).powi(2) + (y2 - y1 as f64).powi(2) as f64).sqrt();

    println!("{:.4}", result);
}
