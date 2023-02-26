#[derive(Debug)]
enum UsState {
    Alaska,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state:{:?}!", state);
            25
        }
    };

    println!("{}", value);
}
