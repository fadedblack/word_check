/*
struct Frequency {
    word      : String,
    frequency : usize,
}
*/

pub struct Book {
    contents: Vec<char>,
    new_words: Vec<String>,
    current_pos : usize,
    end_pos : usize,
}

impl Book {
    pub fn new(path: &str) -> Self {
        let contents = Self::open_file(path);
        let new_words = Vec::new();
        let current_pos = 0;
        let end_pos = contents.len();

        Self {
            contents,
            new_words,
            current_pos,
            end_pos,
        }
    }

    fn open_file(path : &str) -> Vec<char> {

        let file_cont = std::fs::read_to_string(path).unwrap();

        let contents : Vec<char> = file_cont.chars().collect();

        contents
    }

    pub fn get_words(&mut self) -> Vec<String> {
        
        while self.current_pos < self.end_pos {
            let word = self.get_valid_word();

            if !self.checked_word(&word) && Self::is_valid_word_len(&word) {
                self.new_words.push(word.to_string());
            }
            self.current_pos += 1;
        }
        self.new_words.to_owned()
    }


    fn get_valid_word(&mut self) -> String {

        let mut valid_word = String::new();

        let mut chars = self.contents[self.current_pos];

        while !self.is_end_word_char() && !self.is_at_end() {

            if self.check_valid_char(chars) {
                valid_word.push(chars);
            }
            else {
                self.current_pos += 1;

                while !self.is_end_word_char() {
                    self.current_pos += 1;
                }
            }
        
            self.current_pos += 1;
            chars = self.contents[self.current_pos];
        }

        valid_word
    }


    fn is_end_word_char(&self) -> bool {
        
        let mut flag = false;
        if self.current_pos >= self.end_pos {flag = true}

        let chars = self.contents[self.current_pos];

        if chars.is_whitespace() {flag = true}
        else if Self::is_punctuation(chars) {flag = true}
        else if chars == '-' {flag = false}
        else {flag = false}

        return flag;

    }


    fn is_at_end(&self) -> bool {
        if self.current_pos < self.end_pos {
            return true;
        }
        return false;
    }

    fn is_punctuation(chars : char) -> bool {
    
        match chars {
            '-' => false,
            _ if chars.is_ascii_punctuation() => true,
            _ if chars.is_ascii_control() => true,
            _ => false,
        }
    }


    fn check_valid_char(&self, chars : char) -> bool {
        
        if chars.is_ascii_uppercase() {false}
        else if chars.is_ascii_lowercase() {true}
        else {false} 
        
    }


    fn checked_word(&self, word: &str) -> bool {
        if self.new_words.contains(&word.to_string()){
            true
        } else {
            false
        }
    }


    fn is_valid_word_len(word: &str) -> bool {
        if word.len() >= 0 && word.len() <= 3 {
            return false;
        }

        return true;
    }
}
