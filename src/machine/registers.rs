pub struct Register {
    pub value: u8,
    pub base_name: String,
    pub name: String,
}

impl Register {
    pub fn new(index: usize, name: Option<&str>) -> Register {
        let base_name = format!("r{}", index);

        Register {
            value: 0,
            name: name.unwrap_or(&base_name).to_string(),
            base_name: base_name,
        }
    }
}
