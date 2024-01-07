use byte_flow::core;

fn main() {
    if let Err(e) = core::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
