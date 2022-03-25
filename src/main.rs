// The allow is applicable for crate(entire file) since it has macro [!]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// This allow is only associated to the functions
#[allow(unused_imports)]
#[allow(unused_mut)]

use std::io::{stdout, stdin, BufWriter};
use ferris_says::say; // from the dependencies in toml file

/// main function for this crate(module).
fn main() {
    //! # Main function
    //! ```
    //! fn main()
    //! ```
    //! Reads user name as input and generates message by ferris.

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
    printing_basics();
    variable_basics();
}

/// function showcasing different printing techniques in Rust.
fn printing_basics() {
    //! # Printing Basics
    //! ```
    //! fn printing_basics()
    //! ```
    //! This function shows all printing basics with examples.
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    println!("Printing Basics");
    // Formatting
    println!("My name is {} and I am {} years old", "John", 29);
    // Expressions
    println!("a + b = {}", 2+5);
    // Positional arguments
    println!("{0} has a {1} and {0} has a {2}", "Alex", "cat", "dog");
    // Named arguments®
    println!("{name} {surname}", surname="Smith", name="Alex");
    // Printing traits
    println!("{0} decimal is Binary {:b}, Hex {:x}, Octal {:o}", 50, 50, 50);
    // Debug i.e. print complex structures
    println!("Array {:?}", [1, 2, 3]);
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
}

/// function with different ways to handle variables
fn variable_basics() {
    //! # Variable Basics
    //! ```
    //! fn variable_basics()
    //! ```
    //! This function shows all ways to handle mutable/immutable variables.
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    println!("Variable Basics");
    let name = "Maximus";
    let age = 33;
    // Specifying type explicitly to accommodate large int capacity
    let salary:i64 = 150000000;
    /*
    By default the variables in Rust are immutable hence the below commented code will result in
    compile time error.
     */
    // age = 35;
    let mut new_age = 33;
    println!("Name: {name}\tAge: {age}\t mutable New Age: {new_age}\tSalary: {salary}");
    new_age = 35;
    println!("After updating new_age variable value");
    println!("Name: {name}\tAge: {age}\t mutable New Age: {new_age}\tSalary: {salary}");
    // Shadowing is allowed with different types as well and not only of same type
    let color = "Red";
    println!("Color: {color}");
    let color = 222;
    println!("Color: {color}");
    //Declaring multiple variables simultaneously
    let(a, b, c) = (1, "abc", 2.345);
    println!("a: {a}, b: {b}, c: {c}");
    // Commenting below statement since Rust will not do automatic type conversion
    // let pi: f32 = 4;
    println!("pi: f32 = 4 will result in error since mismatched type");
    // Number separator. It will print without separator but only for improving program readability.
    let amount = 1_000_000;
    println!("Amount without number separators {}", amount);
    let is_day = true;
    println!("Boolean is_day {is_day}");
    let character = 'A';
    println!("Single character {character}");
    let emoji_char = '\u{1F601}';
    println!("Emoji character {emoji_char}");
    // String slices are variables that are defined as &str or &'static str.
    // String slices are immutable but can be made mutable using mut keyword.
    let cat: &'static str = "Fluffy";
    println!("&str slice {cat}");
    let mut cats = "Fluffies";
    cats = "Fluffiess";
    println!("Mutable &str slice {cats}");
    // String objects are mutable by default
    let dog = String::new();
    let mut dog = String::from("Rex");
    println!("String object {}", dog);
    println!("Length of &str {} and String {}", cat.len(), dog.len());
    // These functions are not applicable to string slices
    dog.push(' ');
    dog.push_str(" is a Dog");
    let msg = format!("{} who is faithful", dog);
    println!("Print using format!() macro {}", msg);
    /*
    Constant needs to be in upper case, define data type, shadowing is not permitted and can be
    global or local scope
     */
    const URL: &str = "www.google.com";
    // const URL : &str = "www.yahoo.com";
    println!("Constant example {}", URL);
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
}
