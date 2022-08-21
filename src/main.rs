use rand::Rng;

mod shell;
mod bank_os;

fn main() {
    /*
    Zadání:
    1. Konto
    2. Půjčka
        - Max 1
    3. Posílání, příjímání
     */
    let mut input: String;
    println!("Welcome to BankOS\n");
    print_secret_id();
    loop {
        // Get input
        input = shell::get_input();
        // To console
        shell::parse(&input);
        println!("__________\n");
    }
}

fn print_secret_id() {
    println!("{}", rand::thread_rng().gen_range(1..=100));
}