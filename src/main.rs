mod asst;

// To Do List:
// ? Change the wordlist to a vector of String rather than
// a vector of u8.
//
// Write some tests
//
// Check for error
//
// Create a documentation
//
// ?maybe store the new_words in a database

fn main() -> Result<(), std::io::Error> {
    let mut contents = Vec::new();
    let path = "./src/input/Metamorphosis.txt";
    let mut book = asst::therock::Book::new(&path, &mut contents);
    book.get_words();

    let mut index = 1;
    for word in &contents {
        println!("{}: {:?}", index, word);
        index += 1;
    }
    contents.clear();

    Ok(())
}

