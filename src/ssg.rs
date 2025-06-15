use crate::error::*;

use std::fs;

pub struct LyWebpage {
    pub contents: String,
}

impl LyWebpage {
    pub fn read_file(filepath: &str) -> Result<LyWebpage, LyError> {
        Ok(LyWebpage {
            contents: fs::read_to_string(filepath)?,
        })
    }

    pub fn fill_template(mut self, key: &str, contents: &str) -> Self {
        self.contents = self.contents.replace(&format!("[[{key}]]"), contents);
        self
    }
}
