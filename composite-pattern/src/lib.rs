#![allow(dead_code)]

//! 组合模式最终要构造一棵对象树
//! 构建对象树最常用的方式就是递归 (json xml html)

mod file;
mod folder;

pub trait Component {
    fn search(&self, keyword: &str);
}

#[test]
fn component() {
    use file::File;
    use folder::Folder;

    let file1 = File::new("File1");
    let file2 = File::new("File2");
    let file3 = File::new("File3");

    let mut folder1 = Folder::new("Folder1");
    folder1.add_component(file1);

    let mut folder2 = Folder::new("Folder2");
    folder2.add_component(file2);
    folder2.add_component(file3);
    folder2.add_component(folder1);

    folder2.search("rust");
}
