use std::{fs, io::Error};

pub fn get_lines_from_file(path: &str) -> Result<Vec<String>, Error> {
    let contents = fs::read_to_string(path)?;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
