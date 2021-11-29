use std::io;

fn main() {
    let mut name = String::new();
    let mut salary = String::new();
    let mut sales = String::new();

    io::stdin().read_line(&mut name).expect("");
    io::stdin().read_line(&mut salary).expect("");
    io::stdin().read_line(&mut sales).expect("");

    let salary: f64 = salary.trim().parse().expect("");
    let sales: f64 = sales.trim().parse().expect("");
    let result: f64 = salary + (sales * 0.15);

    println!("TOTAL = R$ {:.2}", result);
}
