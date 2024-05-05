/*mod asst;

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

    for word in &contents {
        println!("{:?}", word);
    }

    Ok(())
}
*/
use pdfplumber;
use std::fs::File;
use std::io::Write;

fn main() {
    let pdf_path = "D:/Books/The Metamorphosis(1971).pdf";
    let txt_path = "./src/input/output.txt";

    // Open the PDF file
    let pdf = pdfplumber::open(pdf_path).unwrap();

    // Extract the text from the PDF
    let text = pdf.pages.iter().map(|page| page.extract_text()).collect::<Vec<_>>().join("\n");

    // Write the text to the output file
    let mut file = File::create(txt_path).unwrap();
    file.write_all(text.as_bytes()).unwrap();

    println!("PDF converted to text file: {}", txt_path);
}
