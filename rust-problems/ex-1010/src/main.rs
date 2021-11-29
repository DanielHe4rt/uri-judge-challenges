use std::io;

fn main() {
    let mut items = vec![String::new(); 2];
    let mut result: f64 = 0.00;

    io::stdin().read_line(&mut items[0]).expect("");
    io::stdin().read_line(&mut items[1]).expect("");
    
    for item in items {
        let mut payload = item.split(" ");
        payload.next();

        let amount = payload.next().expect("");
        let price = payload.next().expect("");
        let amount: f64 = amount.trim().parse().expect("");
        let price: f64 = price.trim().parse().expect("");

        result += amount * price;
    }

    println!("VALOR A PAGAR: R$ {:.2}", result);


}
