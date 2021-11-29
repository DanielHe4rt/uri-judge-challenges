use std::io;

fn main() {
    let mut travellingTime = String::new();
    let mut avgSpeed = String::new();

    io::stdin().read_line(&mut travellingTime).expect("");
    io::stdin().read_line(&mut avgSpeed).expect("");
    let travellingTime: f64 = travellingTime.trim().parse().expect("");
    let avgSpeed: f64 = avgSpeed.trim().parse().expect("");

    let outgoing: f64 = travellingTime * avgSpeed / 12.0;

    println!("{:.3}", outgoing);
}
