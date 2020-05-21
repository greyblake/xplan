use xplan::cli;

fn main() {
    cli::run().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
