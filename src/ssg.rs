use regex::Regex;

use crate::error::*;

use std::fs;
use std::io::{self, BufRead};

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

    pub fn fill_from_file(mut self, filepath: &str) -> Result<Self, LyError> {
        let key_pattern = Regex::new(r"\[\[(.*)\]\]")?;

        let file = fs::File::open(filepath)?;

        let mut key = String::new();
        let mut content = String::new();
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                // if the line matches the pattern [[something]] then this is a new key,
                // otherwise it is a line of content for the key already set.
                match key_pattern.captures(&l) {
                    Some(c) => if let Some(k) = c.get(0) {
                        if !key.is_empty() {
                            self.contents = self.contents.replace(&format!("[[{key}]]"), &content);
                            content.clear();
                        }
                        key = k.as_str().to_string();
                    },
                    None => {
                        content += &l;
                    }
                }
            }
        }

        Ok(self)
    }
}
