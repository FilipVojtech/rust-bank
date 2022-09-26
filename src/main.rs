mod shell;
mod bank_os;
mod strings;
mod file_sys;

fn main() {
    /*
    Zadání:
    1. Konto
    2. Půjčka
        - Max 1
    3. Posílání, příjímání
     */
    let mut input: Vec<String>;
    bank_os::init();
    loop {
        // Get input
        input = shell::get_input();
        // To console
        shell::parse_command(input);
        println!("__________\n");
    }
}
