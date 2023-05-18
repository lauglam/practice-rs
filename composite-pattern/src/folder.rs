use crate::Component;

pub struct Folder<'a> {
    name: &'a str,
    children: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, children: vec![] }
    }

    pub fn add_component(&mut self, component: impl Component + 'a) {
        self.children.push(Box::new(component));
    }
}

impl<'a> Component for Folder<'a> {
    fn search(&self, keyword: &str) {
        println!("Searching for keyword {} in folder {}", keyword, self.name);
        self.children.iter().for_each(|c| c.search(keyword));
    }
}
