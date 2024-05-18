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
    contents: Vec<String>,
    new_words: Vec<String>,
}

impl<'a> Book {
    pub fn new(path: &'a str) -> Self {
        let contents = Self::open_file(path);
        let new_words = Vec::new();

        Self {
            contents,
            new_words,
        }
    }

    fn open_file(path : &str) -> Vec<String> {

        let file_cont = std::fs::read_to_string(path).unwrap();

        let contents  = file_cont.split_whitespace().map(|s| s.to_string()).collect();

        contents
    }

    pub fn get_words(&mut self) -> Vec<String> {
        for words in &self.contents {
            let word = &self.get_valid_word(words);

            if self.check_conditions(word) {
                self.new_words.push(word.to_string());
            }
        }
        self.new_words.to_owned()
    }


    fn get_valid_word(&self, word : &String) -> String {

        let valid_word = word;
        
        for (index,characs) in word.chars().enumerate() {
            let flag = self.check_valid_char(characs);

            match flag {
                Flags::INVALID => valid_word.to_string().remove(index),
                _ => continue,
            };
        }

        valid_word.to_string()
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


    fn check_conditions(&self, word : &str) -> bool {
        let flag = Self::is_valid_word_len(&word) && word.is_ascii() && !self.is_checked_word(&word) && !self.is_noun(&word);

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
