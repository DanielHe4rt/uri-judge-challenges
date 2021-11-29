use std::io;

fn main() {
    let mut input = String::new();

    let mut one_notes = 0;
    let mut two_notes = 0;
    let mut five_notes = 0;
    let mut ten_notes = 0;
    let mut twenty_notes = 0;
    let mut fifty_notes = 0;
    let mut hundred_notes = 0;

    io::stdin().read_line(&mut input).expect("");

    let mut input: i32 = input.trim().parse().expect("");
    let money  = input;

    while input > 0 {
        if input >= 100 {
            hundred_notes += 1;
            input -= 100;
            continue;
        }

        if input >= 50 {
            fifty_notes += 1;
            input -= 50;
            continue;
        }

        if input >= 20 {
            twenty_notes += 1;
            input -= 20;
            continue;
        }

        if input >= 10 {
            ten_notes += 1;
            input -= 10;
            continue;
        }

        if input >= 5 {
            five_notes += 1;
            input -= 5;
            continue;
        }

        if input >= 2 {
            two_notes += 1;
            input -= 2;
            continue;
        }

        if input >= 1 {
            one_notes += 1;
            input -= 1;
            continue;
        }
    }

    println!("{}", money);
    println!("{} nota(s) de R$ 100,00", hundred_notes);
    println!("{} nota(s) de R$ 50,00", fifty_notes);
    println!("{} nota(s) de R$ 20,00", twenty_notes);
    println!("{} nota(s) de R$ 10,00", ten_notes);
    println!("{} nota(s) de R$ 5,00", five_notes);
    println!("{} nota(s) de R$ 2,00", two_notes);
    println!("{} nota(s) de R$ 1,00", one_notes);
}
