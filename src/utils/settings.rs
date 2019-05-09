pub struct Settings {
    pub verbose: u64,
    pub log_file: Option<String>,
    pub log_to_stdout: bool,
    pub in_file: String,
    pub memory_size: usize,
}
