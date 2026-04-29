use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut line_count = 1;
    let mut input = String::new();
    println!("--- Rust CLI: Type something and press Enter (Type 'exit' to quit) ---");
    loop {
        print!("{}. ", line_count);
        
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }
        line_count += 1;
    }

    Ok(())
}
