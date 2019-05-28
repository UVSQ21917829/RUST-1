use std::path::{self, Path};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Error};
pub fn ecrire_dans_un_fichier(){
    println!("ecrire le nom du fichier: ");

    let mut com = String::new();
    io::stdin().read_line(&mut com)
       .ok()
       .expect("failed");

    let mut output = com.trim_right();

        let filename = output.to_string();
        let format = ".txt".to_string();
        let file_name = filename + &format;

        let mut text = String::new();
        io::stdin().read_line(&mut text)
       .ok()
       .expect("failed");

        let path = Path::new(&file_name);

        let mut options = OpenOptions::new();
        options.write(true);
        let file: Result<File, Error> = options.open(path);

        let file = match options.open(&path) {
            Ok(file) => file,
            Err(..) => panic!("File does not exist"),
        };


        let mut writer = BufWriter::new(&file);
        writer.write_all(text.as_bytes()).unwrap();;

    }
