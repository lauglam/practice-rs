use crate::Component;

pub struct File<'a> {
    name: &'a str,
}

impl<'a> File<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> Component for File<'a> {
    fn search(&self, keyword: &str) {
        println!("Searching for keyword {} in file {}", keyword, self.name);
    }
}
