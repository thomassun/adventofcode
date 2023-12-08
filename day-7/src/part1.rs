use day_7::{find_item, MyPath, FS};
use regex::Regex;
use std::{error::Error, fs};
fn main() -> Result<(), Box<dyn Error>> {
    let path = "/Users/guoqingsun/code/adventofcode/day-7/data/input.txt";
    let console_output = fs::read_to_string(path)?;
    let mut root = FS::Dir {
        name: "/".to_owned(),
        sub_dir: vec![],
    };
    let mut cwd = MyPath::new();
    let mut cwd_item = Some(&mut root);
    let file_matcher = Regex::new(r"(\d+) (\w+)").unwrap();
    for a in console_output.lines() {
        if a.starts_with('$') {
            //command
            if a.starts_with("$ cd") {
                let re = Regex::new(r"\$ cd (\w+)").unwrap();
                let caps = re.captures(a).unwrap();
                let dir_name = caps.get(0).unwrap().as_str();
                cwd = cwd.go_down(dir_name);
                // let cwd = format!("{}{}/", cwd, dir_name);
                cwd_item = find_item(&mut root, &cwd);
            }
        }
        if a.starts_with("$ ls") {
            continue;
        }
        if a.starts_with("dir") {
            let re = Regex::new(r"dir (\w+)").unwrap();
            let caps = re.captures(a).unwrap();
            let dir_name = caps.get(0).unwrap().as_str();

            let new_dir = FS::Dir {
                name: dir_name.to_owned(),
                sub_dir: vec![],
            };
            if let Some(FS::Dir { sub_dir, .. }) = cwd_item {
                sub_dir.push(new_dir)
            }
        }
        if file_matcher.is_match(a) {
            let caps = file_matcher.captures(a).unwrap();
            let file_name = caps.get(1).unwrap().as_str();
            let size = caps.get(0).unwrap().as_str().parse::<usize>().unwrap();
            let new_file = FS::File {
                name: file_name.to_owned(),
                size,
            };
            if let Some(FS::Dir { sub_dir, .. }) = cwd_item {
                sub_dir.push(new_file);
            }
        }
    }
    Ok(())
}
