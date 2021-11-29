use std::io;

fn main() {
    let mut r = String::new();

    io::stdin().read_line(&mut r).expect("");
    let r: f64 = r.trim().parse().expect("");
    let result: f64 = (4.0/3.0) * 3.14159 * r.powi(3);

    println!("VOLUME = {:.3}", result);
}
