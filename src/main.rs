use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your input: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
        },
        Err(e) => {
            println!("Error encountered when capturing input {}", e);
        }
    }
}
