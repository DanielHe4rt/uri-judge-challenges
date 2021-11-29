use std::io;

fn main() {
    let mut distance = String::new();
    let mut gas = String::new();

    io::stdin().read_line(&mut distance).expect("");
    io::stdin().read_line(&mut gas).expect("");

    let distance: f64 = distance.trim().parse().expect("");
    let gas: f64 = gas.trim().parse().expect("");
    let result: f64 = distance / gas;

    println!("{:.3} km/l", result);
}
