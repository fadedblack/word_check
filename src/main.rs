mod asst;

// To Do List:
// ? Change the wordlist to a vector of String rather than 
// a vector of u8.
//
// Check if word doesn't exist in word list:
// add 'ed' or 'ing' to the word and check it again.
// OR
// Check whether the word is a substring of an word in the list.
// It should check from the 1st till the word.len().
// If matches, check by adding 'ed' or 'ing'.
//
// Known words list is not a good list
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

    Ok(())
}
