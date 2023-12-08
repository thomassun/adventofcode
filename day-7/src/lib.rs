#![allow(clippy::new_without_default)]

pub struct MyPath(String);
impl MyPath {
    pub fn new() -> Self {
        MyPath("/".to_owned())
    }
    pub fn go_up(&self) -> MyPath {
        // from /a/b/c -> /a/b
        if self.is_root() {
            MyPath("/".to_owned())
        } else {
            // /a/b/c
            // /a
            // /
            let path = &self.0.split('/').filter(|c| c != &"").collect::<Vec<_>>();
            if path.len() == 1 {
                MyPath("/".to_owned())
            } else {
                let parent = path[..path.len() - 1].join("/");
                MyPath("/".to_owned() + &parent)
            }
        }
    }
    pub fn destrct(&self) -> Vec<String> {
        //from /a/b/c -> ["/","/a","/a/b","/a/b/c"]
        let path = &self.0.split('/').filter(|c| c != &"").collect::<Vec<_>>();
        let mut result = vec![];
        result.push("/".to_owned());
        // let mut n = 0;
        for (n, p) in path.iter().enumerate() {
            if n == 0 {
                result.push(result[n].to_owned() + p);
            } else {
                result.push(result[n].to_owned() + "/" + p);
            }
        }
        result
    }
    pub fn is_root(&self) -> bool {
        self.0 == "/"
    }
    pub fn go_down(&self, target: &str) -> MyPath {
        // from /a/b/c -> /a/b/c/d
        let current = self.0.to_owned();
        if self.is_root() {
            MyPath(current + target)
        } else {
            MyPath(current + "/" + target)
        }
    }
    pub fn go_next(&self, target: &str) -> MyPath {
        // from /a/b/c -> /a/b/d
        if self.is_root() {
            MyPath("/".to_owned())
        } else {
            self.go_up().go_down(target)
        }
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
    // let paths = ["/","/abc","/abc/def"]
    let paths = path.destrct();
    if let FS::Dir { name, .. } = root {
        if name.as_str() == path.as_str() {
            return Some(root);
        }
    }
    let mut cwd = Some(root);
    for target in paths[1..].iter() {
        if let Some(FS::Dir {
            name: _,
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let mut path = MyPath::new();
        assert!(path.is_root());
        path = path.go_up();
        assert_eq!(path.as_str(), "/");
        path = path.go_down("a");
        assert_eq!(path.as_str(), "/a");
        path = path.go_down("b");
        assert_eq!(path.as_str(), "/a/b");
        path = path.go_down("c");
        assert_eq!(path.as_str(), "/a/b/c");
        path = path.go_next("d");
        assert_eq!(path.as_str(), "/a/b/d");
        path = path.go_up();
        assert_eq!(path.as_str(), "/a/b");
        path = path.go_up();
        assert_eq!(path.as_str(), "/a");
        path = path.go_up();
        assert_eq!(path.as_str(), "/");
        path = path.go_up();
        assert_eq!(path.as_str(), "/");
        path = path.go_next("s");
        assert_eq!(path.as_str(), "/");
        path = path.go_down("a");
        assert_eq!(path.as_str(), "/a");
        path = path.go_next("s");
        assert_eq!(path.as_str(), "/s");
        path = MyPath::new();
        path = path.go_down("a");
        path = path.go_down("b");
        path = path.go_down("c");
        assert_eq!(path.as_str(), "/a/b/c");
        assert_eq!(path.destrct(), vec!["/", "/a", "/a/b", "/a/b/c"])
    }
    #[test]
    fn filesystem() {
        let mut fs = FS::Dir {
            name: "/".to_owned(),
            sub_dir: vec![FS::Dir {
                name: "/a".to_owned(),
                sub_dir: vec![FS::Dir {
                    name: "/a/b".to_owned(),
                    sub_dir: vec![
                        FS::File {
                            name: "xcv".to_owned(),
                            size: 33,
                        },
                        FS::File {
                            name: "abc".to_owned(),
                            size: 313,
                        },
                        FS::File {
                            name: "def".to_owned(),
                            size: 55,
                        },
                        FS::Dir {
                            name: "/a/b/z".to_owned(),
                            sub_dir: vec![],
                        },
                        FS::Dir {
                            name: "/a/b/c".to_owned(),
                            sub_dir: vec![
                                FS::File {
                                    name: "aa.txt".to_owned(),
                                    size: 1311,
                                },
                                FS::File {
                                    name: "bb.txt".to_owned(),
                                    size: 1131,
                                },
                                FS::File {
                                    name: "cc.txt".to_owned(),
                                    size: 1111,
                                },
                            ],
                        },
                        FS::File {
                            name: "ZZZ".to_owned(),
                            size: 199,
                        },
                        FS::File {
                            name: "YYYS".to_owned(),
                            size: 199,
                        },
                    ],
                }],
            }],
        };
        let path = MyPath::new().go_down("a").go_down("b").go_down("c");
        assert!(find_item(&mut fs, &path).is_some());
        let path = path.go_next("z");
        assert!(find_item(&mut fs, &path).is_some());
        let path = path.go_up();
        assert!(find_item(&mut fs, &path).is_some());
        let path = path.go_up();
        assert!(find_item(&mut fs, &path).is_some());
        let path = path.go_next("x");
        assert!(find_item(&mut fs, &path).is_none());
        let path = MyPath::new();
        assert!(find_item(&mut fs, &path).is_some());
    }
}
