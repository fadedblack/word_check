use crate::asst::known_words::KnownWords;
use std::ascii::AsciiExt;
use std::fs::File;
use std::io::Read;

pub struct Book<'a> {
    path: &'a str,
    contents: Vec<u8>,
    new_words: &'a mut Vec<String>,
    all_words: Vec<String>,
    current_pos: usize,
}

impl<'a> Book<'a> {
    pub fn new(path: &'a str, new_words: &'a mut Vec<String>) -> Self {
        let contents = Vec::new();
        let current_pos = 0;
        let all_words = KnownWords::new().contents;
        Self {
            path,
            contents,
            new_words,
            current_pos,
            all_words,
        }
    }

    fn open_file(&mut self) -> Result<usize, std::io::Error> {
        let file = File::open(self.path).expect("Error: File couldn't be open");
        let mut buffer = std::io::BufReader::new(file);
        let no_bytes = buffer.read_to_end(&mut self.contents)?;

        Ok(no_bytes)
    }

    pub fn get_words(&mut self) {
        self.open_file();
        while !self.is_at_end() {
            let mut word = self.get_word().to_ascii_lowercase();

            if Self::is_valid_word_len(&word)
                && word.is_ascii()
                && self.is_checked_word(&word)
                && self.is_known_word(&word) == false
            {
                word = self.get_new_word(&word);
                self.new_words.push(word);
            }
        }
    }

    fn is_checked_word(&self, word: &str) -> bool {
        if self.new_words.contains(&word.to_string()) {
            false
        } else {
            true
        }
    }

    // @Refactor: Can refactor this shit
    fn get_word(&mut self) -> String {
        let mut buffer = String::new();
        for index in self.current_pos..self.contents.len() {
            if buffer.len() > 0 {
                if self.peek(index) == ' ' {
                    self.current_pos += 1;
                    return buffer;
                } else if self.peek(index).is_ascii_punctuation() {
                    self.current_pos += 1;
                    return buffer;
                } else if self.is_at_end() {
                    return buffer;
                } else if self.is_escape_sq() {
                    self.current_pos += 1;
                    return buffer;
                } else if self.peek(index).is_ascii_digit() {
                    self.current_pos += 1;
                } else {
                    buffer.push(self.peek(index));
                    self.current_pos += 1;
                }
            }else {
                if self.peek(index).is_ascii() {
                    buffer.push(self.peek(index));
                    self.current_pos += 1;
                } else {
                    self.current_pos += 1;
                }
            }
        }
        buffer.clear();
        return '\0'.to_string();
    }

    fn is_at_end(&self) -> bool {
        if self.current_pos >= self.contents.len() {
            return true;
        }
        return false;
    }

    fn peek(&self, index: usize) -> char {
        self.contents[index] as char
    }

    fn is_valid_word_len(word: &str) -> bool {
        if word.len() >= 1 && word.len() <= 3 {
            return false;
        }
        return true;
    }

    fn is_escape_sq(&self) -> bool {
        match self.contents[self.current_pos] {
            b'\n' => true,
            b'\t' => true,
            b'\r' => true,
            b'\\' => true,
            b'\"' => true,
            b'\'' => true,
            b'\0' => true,
            _ => false,
        }
    }

    // @Cleanup: Doing same thing as is_known_word 
    fn is_known_word(&self, word: &str) -> bool {
        let mut new_word = true;
        for known_word in &self.all_words {
            if let Some(char_index) = known_word.find(&word) {
                if char_index == 0 && word.len() == known_word.len() {
                    return new_word;
                } else {
                    if char_index == 0 {
                        let last_pos = word.len() - 1;
                        let second_last= last_pos - 1;
                        let third_last= last_pos - 2;

                        if word.chars().nth(third_last as usize).unwrap() == 'i' 
                        && word.chars().nth(second_last as usize).unwrap() == 'n' 
                        && word.chars().nth(last_pos as usize).unwrap() == 'g' {
                            return new_word;
                        } else if word.chars().nth(second_last as usize).unwrap() == 'e' 
                        && word.chars().nth(last_pos as usize).unwrap() == 'd' {
                            return new_word;
                        }
                    } else {
                        new_word = false;
                    }
                }
            } else {
                new_word = false;
                return new_word;
            }
        }
        return new_word;
    }

    fn get_new_word(&self, word: &str) -> String {
        let last_pos: usize    = word.chars().count() - 1;
        let second_last: usize = last_pos - 1;
        let third_last: usize  = last_pos - 2;

        if word.chars().nth(third_last).unwrap() == 'i' 
        && word.chars().nth(second_last).unwrap() == 'n' 
        && word.chars().nth(last_pos).unwrap() == 'g' {
            word.to_string().pop();
            word.to_string().pop();
            word.to_string().pop();
            return word.to_string();
        } else if word.chars().nth(second_last).unwrap() == 'e' && word.chars().nth(last_pos).unwrap() == 'd' {
            word.to_string().pop();
            word.to_string().pop();
            return word.to_string();
        } else {
            return word.to_string();
        }
    }
}
