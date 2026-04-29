use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut line_count = 1;
    let mut input = String::new();
    let log_file_path = "history.txt";

    // Setup Colors
    let blue = "\x1b[34m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    println!("{}======================================={}", blue, reset);
    println!("  {}Rust CLI v1.1{} - Logging to {}history.txt{}", green, reset, yellow, reset);
    println!("{}======================================={}", blue, reset);

    loop {
        print!("{}[{}]{} > ", blue, line_count, reset);
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.is_empty() { continue; }

        match trimmed.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("{}Goodbye!{}", yellow, reset);
                break;
            }
            "help" => {
                println!("\n{}Commands:{} help, clear, exit\n", green, reset);
            }
            "clear" => {
                print!("{}[2J{}[1;1H", 27 as char, 27 as char);
            }
            _ => {
                // --- FILE LOGGING FEATURE ---
                // Open file in append mode. Create it if it doesn't exist.
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_file_path)?;

                // Write the line number and user input to the file
                writeln!(file, "Line {}: {}", line_count, trimmed)?;
                
                println!("{}Logged: {}{}", yellow, trimmed, reset);
            }
        }
        line_count += 1;
    }
    Ok(())
}
