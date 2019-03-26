## Les Bibliothéque tierce en RUST

 ### Cargo

Cargo est le gestionnaire de colis Rust . Cargo télécharge les dépendances de votre paquet Rust, compile vos paquets, crée des paquets distribuables et les télécharge sur [crates.io](https://crates.io/) , le registre de paquets de la communauté Rust . 

  
  Commençons par le fichier **Cargo.toml**, ajoutez ces deux lignes :
```markdown 
[dependencies]
time = "0.1"
```

Nous avons donc ajouté une dépendance vers la bibliothèque time. Maintenant dans votre fichier principal (celui que vous avez indiqué à Cargo), ajoutez :
```markdown 
extern crate time;
```

Pour appeler une fonction depuis la bibliothèque, il suffit de faire :

```markdown 
println!("{:?}", time::now());
```

Et c'est tout ! Les imports fonctionnent de la même façon :

```markdown
use time::Tm;
```
