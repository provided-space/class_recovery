#[derive(Clone)]
pub struct ClassBuffer<'a> {
    contents: &'a [u8],
    name: String,
}

impl<'a> ClassBuffer<'a> {

    pub fn new(contents: &'a [u8], name: String) -> Self {
        return ClassBuffer { contents, name };
    }

    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn len(&self) -> usize {
        return self.contents.len();
    }

    pub fn get_contents(&self) -> &'a [u8] {
        return self.contents;
    }
}
