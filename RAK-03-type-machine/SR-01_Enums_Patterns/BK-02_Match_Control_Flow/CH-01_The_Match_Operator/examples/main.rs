enum UsState {
    Alabama,
    Alaska,
    // ... lainnya
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Enum di dalam Enum
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter dari negara bagian: {:?}", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("Nilainya: {} sen", value_in_cents(c));
}
