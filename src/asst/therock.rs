use std::fs::File;
use std::io::Read;


pub struct Book<'a> {
    path : &'a str,
    contents : String,
    new_words : Vec<String>,
    current_pos : usize
}


impl <'a> Book <'a> {
    pub fn new(path : &'a str) -> Self {
        let contents = String::new();
        let new_words = Vec::new();
        let current_pos = 0;
        Self {
            path,
            contents,
            new_words,
            current_pos,
        }
    }


    pub fn open_file(&mut self) -> Result<usize, std::io::Error> {
        let mut file = File::open(self.path)
            .expect("Error: File couldn't be open");
        let no_bytes = file.read_to_string(&mut self.contents)?;

        Ok(no_bytes)
    }


    pub fn read_file(&mut self, new_words : &mut Vec<String>) {
        while !self.is_at_end(){
            let word = self.get_word();
            self.current_pos += 1; //should be at the end of loop
            new_words.push(word);
        }
    }


    fn get_word(&mut self) -> String {
        let mut buffer = String::new();
        for chars in self.contents.chars() {
            if chars == ' ' {
                return buffer;
            }
            else if Self::is_punctuation(chars) { 
                return buffer;
            }
            else if self.is_at_end() {
                return buffer;
            } 
            else {
                buffer.push(chars);
            }
        self.current_pos += 1;
        }
        return buffer;
    }


    fn is_at_end(&self) -> bool {
        if self.current_pos >= self.contents.chars().count() {
            return true;
        }
        return false;
    }


    fn is_punctuation(chars : char) -> bool {
        match chars {
            '.' => true,
            ',' => true,
            '?' => true,
            '!' => true,
            ':' => true,
            ';' => true,
            '-' => true,
            '(' => true,
            ')' => true,
            '"' => true,
            _ => false,
        }
    }
}
