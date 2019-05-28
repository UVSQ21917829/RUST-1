use::std::io;
use::std::io::prelude::*;
use::std::fs::File;
mod create;
mod lecture;
mod suppression;
mod liste;
mod ecrire;
fn main(){

    loop {

        println!("faire un choix:
                  1. Creer un fichier:
                  2. ecrire dans un fichier:
                  3. supprimer un fichier:
                  4. lister les fichiers:
                  5. lire un fichier:
                  6. quitter: ");
                  let mut input = String::new();
                  io::stdin().read_line(&mut input)
                      .expect("failed to read line");

                  let input = input.trim().parse::<u32>()
                      .map_err(|_| format!("{} n'est pas un nombre", input))
                      .unwrap();

                      match input {
                             1 => create::create_fichier(),
                             2 => ecrire::ecrire_dans_un_fichier(),
                             3 => suppression::supprimer(),
                             4 => liste::liste_fichier(),
                             5 => lecture::lire_file(),
                             6 => break,
                             _ => {
                                 println!("hmmm, {} , existe pas dans la liste", input);
                                 return;
                             }
                         }
    }

}
