use::std::io;
use::std::io::prelude::*;
use::std::fs::File;
use std::fs;
pub fn supprimer(){
    let mut input = String::new();
    io::stdin().read_line(&mut input)
       .ok()
       .expect("failed to read line");
        let mut output = input.trim_right();
        let filename = output.to_string();
        let format = ".txt".to_string();
        let file_name = filename + &format;
     fs::remove_file(file_name.to_string()).expect(" fichier néexiste pas");
     println!("le ficher, {} , est supprimé", file_name);


}
#[test]
pub fn supprimer_fichier_test(){
    fs::remove_file("saq.txt").expect(" fichier néexiste pas");
    let mut i:i32=0;
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("{:?}", entry.file_name());
                i+=1;
            }
        }
    }
    assert_eq!(i,23);
}
