use std::io::Read;

fn main() {
    let mut input = String::with_capacity(512);

    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read from stdin: {}", e);
        std::process::exit(1);
    }

    // Check the commit message
    if let Err(e) = commit_check_core::length::check(&input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    std::process::exit(0);
}
