mod asst;

// Why is it taking so much time??
// Known words list is not a good list
// Make so that it reads on ascii_characters
fn main() -> Result<(), std::io::Error> {
    use std::time::Instant;
    let now = Instant::now();

    let mut contents = Vec::new();
    let path = "./src/Metamorphosis.txt";
    let mut book = asst::therock::Book::new(&path, &mut contents);
    book.get_words();

    for val in &contents {
        println!("{:?}", std::str::from_utf8(val).unwrap());
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
