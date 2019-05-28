use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Error};
pub fn lire_file() {
    let mut i:u32=0;
    println!("Enter filename: ");

let mut com = String::new();
io::stdin().read_line(&mut com)
   .ok()
   .expect("failed");

let output = com.trim_right();

    let filename = output.to_string();
    let format = ".txt".to_string();
    let file_name = filename + &format;

    let mut file = match File::open(file_name.to_string()) {
        Err(e) => {
    	println!("Error open file {}", e);
    	return;
        },
        Ok(file) => file,
    };

    let mut reader = BufReader::new(&file);

    let buffer_string = &mut String::new();
    reader.read_line(buffer_string);
    println!("le Contenu: {}", buffer_string);
    loop {
        let buffer_string = &mut String::new();
        reader.read_line(buffer_string);
        if buffer_string.len()==0{
            break;
        }else{
            println!("{}", buffer_string);
        }

    }

}
#[test]
pub fn lire_file_test() {
    let mut i:u32=0;

    let mut file = match File::open("moi.txt") {
        Err(e) => {
        println!("Error open file {}", e);
        return;
        },
        Ok(file) => file,
    };

    let mut reader = BufReader::new(&file);
    let buffer_string = &mut String::new();
    reader.read_line(buffer_string);
    assert_eq!(buffer_string,"farhi");
}
