use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut line_count = 1;
    let mut input = String::new();

    // UI Constants for colors (ANSI escape codes)
    let blue = "\x1b[34m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    println!("{}======================================={}", blue, reset);
    println!("  {}Rust CLI v1.0{} - Type '{}help{}' for info", green, reset, yellow, reset);
    println!("{}======================================={}", blue, reset);

    loop {
        // Styled prompt
        print!("{}[{}]{} > ", blue, line_count, reset);
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        match trimmed.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("{}Goodbye!{}", yellow, reset);
                break;
            }
            "help" => {
                println!("\n{}Available Commands:{}", green, reset);
                println!("  help  - Show this menu");
                println!("  clear - (Visual) adds space");
                println!("  exit  - Close the program\n");
            }
            "clear" => {
                print!("{}[2J{}[1;1H", 27 as char, 27 as char); // Standard ANSI clear screen
            }
            "" => continue, // Ignore empty enters
            _ => {
                println!("{}Echo: {}{}", yellow, trimmed, reset);
            }
        }

        line_count += 1;
    }

    Ok(())
}
