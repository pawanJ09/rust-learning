#[allow(unused_imports)]

use std::io::{stdout, stdin, BufWriter};
use ferris_says::say; // from the dependencies in toml file

fn main() {
    let mut input = String::new();
    println!("Enter your input: ");
    match stdin().read_line(&mut input) {
        Ok(_) => {
            let stdout = stdout();
            let input_message = format!("Hello {}", input);
            let message = String::from(input_message);
            let width = message.chars().count();

            let mut writer = BufWriter::new(stdout.lock());
            say(message.as_bytes(), width, &mut writer).unwrap();
        },
        Err(e) => {
            println!("Error encountered when capturing input {}", e);
        }
    }
}
