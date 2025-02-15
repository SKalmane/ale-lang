use std::io::Write;

fn main() {
    println!("Thanks for trying ~~ ale ~~");
    loop {
        print!("~~ ");
        let _ = std::io::stdout().flush();
        // Get user input
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        println!("User input is: {}", user_input);

        // compare user_input to the string "exit" and if so, break out of loop
        if user_input.trim() == "exit" {
            println!("~~ Goodbye ~~");
            break;
        }
    }
}

