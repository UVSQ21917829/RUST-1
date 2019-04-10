[**Base du langage**](https://uvsq21807686.github.io/RUST)-----[**Bibliothèques standards**](https://uvsq21807686.github.io/RUST/std)-----[**Bibliothèques tierces**](https://uvsq21807686.github.io/RUST/trc)-----[**Ressources d'apprentissages**](https://uvsq21807686.github.io/RUST/rsc)  


## Outils de développement 

Pour pouvoir développer sur Rust , il va falloir utiliser le bon outils , pour programmer sur RUST vous pouvez utiliser leur éditeur de code [**play.rust-lang.org**](https://play.rust-lang.org/)

![figure](outils.png)


  Soit [ce lien](http://www.tutorialspoint.com/compile_rust_online.php)
qui permet d'éditer, compiler et exécuter des projets complets (répartis sur plusieurs fichiers) tout en proposant des outils d'import et d'export.

![figure](outils2.png)


Si vous n’utilisez pas l’éditeur en ligne .

Après avoir créer votre fichier.rs sur sublime text ou autre avec le compilateur installer vous n’avez plus compilé le fichier comme ceci :



![figure](outils3.png)

  ![figure](outils4.png)
  
## Le compilateur de Rust
Si vous ne souhaitez pas utiliser l'éditeur Rust en ligne, il va vous falloir télécharger le compilateur de Rust disponible [ici](https://www.rust-lang.org/), puis l'installer.

## Tests Unitaire
  Pour faire une test unitaire en rust il suffit d'ajouter #[test] sur la ligne avant la fonction de test,la fonction ne doit prendre aucun argument en paramétre et ne rien renvoyer.
  Les fonctions check, fail, assert (ainsi que assert_eq, assert_approx_eq, etc) sont très utiles pour les tests unitaires.
  ### Exemple :
  
  ```markdown
  fn somme(x: i32,y :i32)->i32{
      x+y
  }
  #[test]
fn test_somme() {
    assert_eq!(3,somme(1,2));
}
  ```
  
  Si on souhaite que la fonction échoue, il faut mettre en plus #[should_fail].
  ### Exemple :
  ```markdown
  #[test]
  #[should_fail]
fn test_l’échec() {
    assert_eq!(4,somme(1,2));
}
  ```
  Il existe un type de tests unitaires un peu spécial : les benchmarks (tests de performances). Il faut utiliser l’attribut #[bench] mais aussi un peu plus que ça
  
  ### Exemple :
 ```markdown 
  #[bench]
fn test_trucmuche(b: &mut extra::test::BenchHarness) { // on va utiliser l’argument
    // le code de préparation
    do b.iter() {
        // le code dont vous souhaitez mesurer les performances
    }
}
```
