pub struct KnownWords {
    pub contents : Vec<String>,
}

impl  KnownWords {
    pub fn new() -> Self {
        let contents = Self::get_known_words();
        Self {
            contents,
        }
    }

    fn get_known_words() -> Vec<String> {
        let contents = std::fs::read_to_string("./src/input/training.txt").expect("Error: Known Word List couldn't be read");
        let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

        lines
    }
}
