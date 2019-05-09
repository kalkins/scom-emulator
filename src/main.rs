use std::fs;

mod utils;
mod machine;


fn main() {
    let settings = utils::args::parse_arguments();

    let input = fs::read_to_string(&settings.in_file)
        .expect("Something went wrong while reading the input file")
        .into_bytes();

    let mut machine = machine::scom::SCOM::new(&settings);
    machine.load_program(&input);
}
