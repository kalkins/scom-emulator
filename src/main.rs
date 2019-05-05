mod utils;


fn main() {
    let settings = utils::args::parse_arguments();

    println!("Verbosity level: {}", settings.verbose);
    println!("Log to stdout: {}", settings.log_to_stdout);
    println!("Log file: {}", settings.log_file.unwrap_or("-".to_string()));
    println!("Input file: {}", settings.in_file.unwrap_or("-".to_string()));
}
