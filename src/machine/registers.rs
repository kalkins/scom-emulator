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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let reg1 = Register::new(1, None);
        assert_eq!("r1", reg1.name);
        assert_eq!("r1", reg1.base_name);

        let reg2 = Register::new(2, None);
        assert_eq!("r2", reg2.name);
        assert_eq!("r2", reg2.base_name);

        let reg3 = Register::new(3, Some("third"));
        assert_eq!("third", reg3.name);
        assert_eq!("r3", reg3.base_name);
    }
}
