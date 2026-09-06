
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;
    lucky_penny(&coin);
}

fn value_in_cents(coin: &Coin) -> u8{
    match coin  {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn lucky_penny(coin: &Coin) -> bool {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            true
        }
        _ => false
    }
}
