use std::io;

fn main() {
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).expect("");
    let distance: i32 = distance.trim().parse().expect(""); 

    println!("{} minutos", distance * 2);
}
