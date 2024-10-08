use crate::asst::known_words::KnownWords;
use std::fs::File;
use std::io::Read;


pub struct Book<'a> {
    path        : &'a str,
    contents    : Vec<u8>,
    new_words   : &'a mut Vec<String>,
    known_words : Vec<String>,
    current_pos : usize,
}

impl<'a> Book<'a> {
    pub fn new(path: &'a str, new_words: &'a mut Vec<String>) -> Self {

        let contents = Vec::new();
        let current_pos = 0;
        let known_words = KnownWords::new().contents;

        Self {
            path,
            contents,
            new_words,
            current_pos,
            known_words,
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
                && !self.is_checked_word(&word)
                && !self.is_known_word(&mut word)
            {
                self.new_words.push(word);
            }
        }
    }


    fn is_checked_word(&self, word: &str) -> bool {

        if self.new_words.contains(&word.to_string()) {
           true 
        } else {
            false
        }
    }


    fn get_word(&mut self) -> String {

        let mut buffer = String::new();
        
        for index in self.current_pos..self.contents.len() {
            if buffer.len() > 0 {
                if self.get_valid_chars(index) {
                    buffer.push(self.peek(index));
                } else {
                    return buffer;
                }
            } else {
                if self.get_valid_chars(index) {
                    buffer.push(self.peek(index));
                }
            }
        }

        buffer.clear();
        return '\0'.to_string();
    }


    fn get_valid_chars(&mut self, index: usize) -> bool {

        let mut buffer = false;

        if self.peek(index) == ' ' {
            self.current_pos += 1;
        } else if self.peek(index) == '-' {
            self.current_pos += 1;
        } else if self.peek(index).is_ascii_punctuation() {
            self.current_pos += 1;
        } else if self.is_at_end() {
            buffer = true;
        } else if self.is_escape_sq() {
            self.current_pos += 1;
        } else if self.peek(index).is_ascii_digit() {
            self.current_pos += 1;
        } else {
            buffer = true;
            self.current_pos += 1;
        }

        return buffer;
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

    // @Cleanup: Why is there two vectors containing the same thing
    // all_words and contents.
    fn is_known_word(&self, word: &mut str) -> bool {
        let mut new_word = true;
        for known_word in &self.known_words{
            if let Some(char_index) = known_word.find(&*word) {
                if char_index == 0 && word.len() == known_word.len() {
                    return new_word;
                } else {
                    if char_index == 0 {
                        let last_pos = word.len() - 1;
                        let second_last = last_pos - 1;
                        let third_last = last_pos - 2;

                        if word.chars().nth(third_last as usize).unwrap() == 'i'
                            && word.chars().nth(second_last as usize).unwrap() == 'n'
                            && word.chars().nth(last_pos as usize).unwrap() == 'g'
                        {
                            word.to_string().pop();
                            word.to_string().pop();
                            word.to_string().pop();
                            return new_word;
                        } else if word.chars().nth(second_last as usize).unwrap() == 'e'
                            && word.chars().nth(last_pos as usize).unwrap() == 'd'
                        {
                            word.to_string().pop();
                            word.to_string().pop();
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
}
