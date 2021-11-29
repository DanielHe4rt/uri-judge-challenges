use std::io;

fn main() {
    let mut inputs = String::new();
    let mut result: u32 = 0;

    io::stdin().read_line(&mut inputs).expect("");

    let inputs = inputs.split_whitespace();

    for x in inputs {
        let n: u32 = x.trim().parse().expect("");
        if n > result {
            result = n;
        }
    }

    println!("{} eh o maior", result);
}
