mod asst;

// Why is it taking so much time??
// Reading just the first word
fn main() -> Result<(), std::io::Error> {
    let mut book = asst::therock::Book::new("./src/example1.txt");    
    book.open_file().unwrap();
    let mut cont = Vec::new();
   book.read_file(&mut cont);


    for chars in cont {
        print!("{} ", chars);
    }
    //println!("{contents}");
    Ok(())
}
