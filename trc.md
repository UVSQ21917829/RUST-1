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

Pour commencer un project avec Cargo, rien de plus facile :
```markdown
> cargo new mon_nouveau_project
```

Un nouveau dossier s'appelant **mon_nouveau_project** sera crée :
```markdown
- mon_nouveau_project
     |
     |- Cargo.toml
     |- .gitignore
     |- src/
```

Le fichier **Cargo.toml** à la racine de votre projet devrait contenir :
```markdown
[package]
name = "mon_nouveau_project"
version = "0.0.1"
authors = ["Votre nom <vous@exemple.com>"]
```

Tous les fichiers sources (.rs normalement) doivent être placés dans un sous-dossier appelé **src**. C’est à dire qu'on va avoir un fichier **main.rs** dans le dossier **src** :
```markdown
fn main() {
    println!("Début du projet");
}
```

Maintenant pour compiler le projet, il vous suffit de faire :
```markdown
> cargo build
```
Et voilà ! L'exécutable sera généré dans le dossier **target/debug/**. Pour le lancer :

```markdown
> ./target/debug/mon_nouveau_project
Début du projet
```

Si vous voulez compiler et lancer l'exécutable tout de suite après, vous pouvez utiliser la commande **run** :
```markdown
> cargo run
     Fresh mon_nouveau_project v0.0.1 (file:///path/to/project/mon_nouveau_project)
    Running `target/debug/mon_nouveau_project`
Début du projet
```

Et voilà !

Par défaut, **cargo** compile en mode **debug**. Les performances sont **BEAUCOUP** plus faibles qu'en mode **release**, faites bien attention à vérifier que vous n'avez pas compilé en mode **debug** dans un premier temps si vous avez des problèmes de performance. Si vous souhaitez compiler en mode release, il vous faudra passer l'option "--release" :
```markdown
> cargo build --release
```
Bien évidemment, l'exécutable généré se trouvera dans le dossier **target/release**.

Cela fonctionne de la même façon pour lancer l'exécution :
```markdown
> cargo run --release
```



