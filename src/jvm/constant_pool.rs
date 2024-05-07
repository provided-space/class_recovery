use std::collections::HashMap;

pub struct ConstantPool{
    strings: HashMap<u16, String>,
    symbols: HashMap<u16, u16>,
}

impl ConstantPool {
    pub fn new() -> Self {
        return ConstantPool {
            strings: HashMap::new(),
            symbols: HashMap::new(),
        };
    }

    pub fn put_string(&mut self, index: u16, string: String) {
        self.strings.insert(index, string);
    }

    pub fn get_string(&self, index: &u16) -> Option<&String> {
        return self.strings.get(index);
    }

    pub fn put_symbol(&mut self, index: u16, symbol: u16) {
        self.symbols.insert(index, symbol);
    }

    pub fn get_symbol(&self, index: u16) -> Option<&u16> {
        return self.symbols.get(&index);
    }
}
