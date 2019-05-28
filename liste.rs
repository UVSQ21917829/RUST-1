use std::fs;

pub fn liste_fichier(){
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("{:?}", entry.file_name());
            }
        }
    }
}
#[test]
pub fn liste_fichier_test(){
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
    assert_eq!(i,24);
}
