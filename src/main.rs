#[allow(unused_imports)]

use std::io::{stdout, stdin, BufWriter};
use ferris_says::say; // from the dependencies in toml file

/// main function for this crate(module)
fn main() {
    //! # Main function
    //!
    //! ```
    //! fn main()
    //! ```
    //!
    //! Reads user name as input and generates message by ferris.
    //!

    // Creating a mutable string as new
    let mut input = String::new();
    println!("Enter your input: ");
    match stdin().read_line(&mut input) {
        Ok(_) => {
            /*
            Here we are using the input and formatting our welcome message.
            This message will then be used to pass to the ferris-say function.
             */
            let stdout = stdout();
            let input_message = format!("Hello {}", input);
            let message = String::from(input_message);
            let width = message.chars().count();

            let mut writer = BufWriter::new(stdout.lock());
            say(message.as_bytes(), width, &mut writer).unwrap();
        },
        Err(e) => {
            // Print error if input fails
            println!("Error encountered when capturing input {}", e);
        }
    }
}
