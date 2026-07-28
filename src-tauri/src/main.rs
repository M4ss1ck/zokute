fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(cmd) = zokute::cli::parse_args(&args) {
        match zokute::cli::send_command(&cmd) {
            Ok(response) => { println!("{response}"); std::process::exit(0); }
            Err(e) => { eprintln!("{e}"); std::process::exit(1); }
        }
    }
    zokute::run();
}
