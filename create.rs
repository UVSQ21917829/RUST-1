use::std::io;
use::std::io::prelude::*;
use::std::fs::File;
use std::io::LineWriter;
pub fn create_fichier(){
    println!("enter le nom du fichier");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
       .ok()
       .expect("failed to read line");

        let mut output = input.trim_right();
        let filename = output.to_string();
        let format = ".txt".to_string();
        let file_name = filename + &format;
        let mut file=File::create(file_name.to_string()).expect(" problem");
        println!("le fichier, {} , est cree", file_name);

}
