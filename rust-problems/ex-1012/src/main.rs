use std::io;

fn main() {
    let mut values = String::new();

    
    io::stdin().read_line(&mut values).expect("");

    let mut values = values.split_whitespace();
    let a: f64 = values.next().expect("").trim().parse().expect("");
    let b: f64 = values.next().expect("").trim().parse().expect("");
    let c: f64 = values.next().expect("").trim().parse().expect("");

    let triangle_shape: f64 = a * c / 2.0;
    let circle_shape: f64 = c.powi(2) * 3.14159;
    let trapeze_shape: f64 = ((a + b) / 2.0) * c;
    let square_shape: f64 = b.powi(2);
    let rectangle_shape: f64 = a * b;

    println!("TRIANGULO: {:.3}", triangle_shape);
    println!("CIRCULO: {:.3}", circle_shape);
    println!("TRAPEZIO: {:.3}", trapeze_shape);
    println!("QUADRADO: {:.3}", square_shape);
    println!("RETANGULO: {:.3}", rectangle_shape);
}
