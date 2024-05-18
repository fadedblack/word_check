#[allow(unused)]
mod asst;

// To Do List:
//
// Write some tests
//
// Check for error
//
// Create a documentation
//
// ?maybe store the new_words in a database
//
// Word with length less than 3
// Any names of people or places
// - If name starts with capital letter
// - ignore those words
// - store the lowercase version of the same
// - check whether the word is the same
// - 
//

fn main() -> Result<(), std::io::Error> {
    let path = "./src/input/Metamorphosis.txt";
    let mut book = asst::therock::Book::new(&path);
    let contents = book.get_words();

    println!("{:?}", contents);
    Ok(())
}

