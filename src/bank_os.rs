use crate::strings;

struct User {
    name: String,
    email: String,
    balance: f64,
    owed: f64,
}

pub fn init() {
    println!("{}", strings::WELCOME);
    // Create users dir if needed
}

pub fn send(amount: f32, user: &str) {
    println!("amount {:?}", amount);
    println!("user {:?}", user);
    todo!();
}