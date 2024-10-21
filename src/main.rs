fn main() {
    if let Err(err) = houen::run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
