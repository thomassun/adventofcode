#![allow(clippy::new_without_default)]

use std::borrow::BorrowMut;

pub struct MyPath(String);
impl MyPath {
    pub fn new() -> Self {
        MyPath("/".to_owned())
    }
    pub fn go_up(&self) -> MyPath {
        if self.0 == "/" {
            MyPath("/".to_owned())
        } else {
            let path = &self.0;
            MyPath(path.to_owned())
        }
    }
    pub fn destrct(&self) -> Vec<&str> {
        todo!()
    }
    pub fn go_down(&self, target: &str) -> MyPath {
        target.to_owned();
        todo!()
    }
    pub fn go_next(&self, target: &str) -> MyPath {
        todo!()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
pub enum FS {
    File { name: String, size: usize },
    Dir { name: String, sub_dir: Vec<FS> },
}

pub fn find_item<'a>(root: &'a mut FS, path: &MyPath) -> Option<&'a mut FS> {
    // let paths = ["/","/abc/","/abc/def/"]
    let paths = path.destrct();
    if let FS::Dir { name, .. } = root {
        if name.as_str() == path.as_str() {
            return Some(root);
        }
    }
    let mut cwd = Some(root);
    for target in paths {
        if let Some(FS::Dir {
            name,
            ref mut sub_dir,
        }) = cwd
        {
            cwd = sub_dir.iter_mut().find(|f| {
                if let FS::Dir { name, .. } = f {
                    name == target
                } else {
                    false
                }
            })
            // .expect("FINDING NOTHING")
            // .borrow_mut()
        } else {
            panic!("Cannot be here")
        }
    }
    cwd
}

fn destruct_path(path: &str) -> Vec<&str> {
    todo!()
}
