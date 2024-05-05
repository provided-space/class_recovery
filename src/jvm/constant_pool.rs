use std::collections::HashMap;

pub struct ConstantPool{
    strings: HashMap<u16, String>,
}

impl ConstantPool {
    pub fn new() -> Self {
        return ConstantPool {
            strings: HashMap::new(),
        };
    }

    pub fn put_string_at(&mut self, index: u16, string: String) {
        self.strings.insert(index, string);
    }

    pub fn get_string_at(&self, index: u16) -> Option<&String> {
        return self.strings.get(&index);
    }
}
