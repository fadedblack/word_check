use std::str::EncodeUtf16;

#[derive(PartialEq)]
enum Flags {
//    HYPHEN,
    NULL,
    VALID,
    INVALID,
}

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

impl<'a> Book {
    pub fn new(path: &'a str) -> Self {
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
            let word = Self::get_valid_word();

            if self.check_conditions(word) {
                self.new_words.push(word.to_string());
            }
            self.new_words.to_owned()
        }
    }


    fn get_valid_word(&self) -> String {

        let mut valid_word = String::new();
        
        let mut chars = self.contents[self.current_pos];
        while !self.check_conditions(chars) {
            valid_word.push(chars);
        }

        valid_word
    }


    fn check_valid_char(&self, chars : char) -> Flags {
        
        let mut flag = Flags::NULL;

        if chars == '-' {
            flag == Flags::INVALID;
        } else if chars.is_ascii_punctuation() {
            flag == Flags::INVALID;
        } else if Self::is_escape_sq(chars) {
            flag == Flags::INVALID;
        } else if chars.is_ascii_digit() {
            flag == Flags::INVALID;
        }
       
        if flag != Flags::NULL {
            flag = Flags::VALID;
        }

        return flag;
    }


    fn check_conditions(&self, chars : char) -> bool {
        let flag = true;



        flag
    }


    fn is_noun(&self, word : &str) -> bool {
        if word.chars().nth(0).unwrap().is_ascii_uppercase(){
            //self.most_common_words.push(word.to_lowercase());
            return true;
        }
        return false;
    }


    fn is_checked_word(&self, word: &str) -> bool {
        if self.new_words.contains(&word.to_string()) {
            true
        } else {
            false
        }
    }


    fn is_valid_word_len(word: &str) -> bool {
        if word.len() >= 1 && word.len() <= 3 {
            return false;
        }

        return true;
    }


    fn is_escape_sq(chars : char) -> bool {
        match chars {
            '\n' => true,
            '\t' => true,
            '\r' => true,
            '\\' => true,
            '\"' => true,
            '\'' => true,
            '\0' => true,
            _ => false,
        }
    }
}
