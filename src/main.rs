use std::env; // import env so we can read command line args
use std::fs; // file handling

fn main() {
    // collect command line args into a vector of strings and print
    let args: Vec<String> = env::args().collect();
    
    // debug command
    // dbg!(args); 
    
    // set variables using reference from args
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // read file into a string
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");
    
    // debug by printing file
    println!("With text:\n{contents}");

}