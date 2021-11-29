use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("fudeuzão");

    let n: f64 = n.trim().parse().expect("aaaa");

    let pi: f64 = 3.14159;

    let result: f64 = pi * (n * n);

    println!("A={:.1$}", result, 4);
}
