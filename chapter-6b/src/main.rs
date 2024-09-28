enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let mut value = value_in_cents(Coin::Quarter);
    println!("{}", value);
    value = value_in_cents(Coin::Dime);
    println!("{}", value);
    value = value_in_cents(Coin::Nickel);
    println!("{}", value);
    value = value_in_cents(Coin::Penny);
    println!("{}", value);
}
