#[derive(Debug)] // Enables {:?} for printing
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel, Dime(UsState), Quarter(UsState), }

fn value_in_cents(coin: Coin) -> u8 { match coin { Coin::Penny => 1, Coin::Nickel => 5,
        Coin::Dime(state) => {
            println!("State dime from {:?}!", state);
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Coin value in cents: {}", value);
    let coin = Coin::Dime(UsState::Alabama);
    let value = value_in_cents(coin);
    println!("Coin value in cents: {}", value);
}
