## Bienvenue sur ma page 
## Base du langage

## 1.1 introduction

 Rust est un langage de programmation système moderne mettant l'accent sur la sécurité, la vélocité et la concurrence, compilé et multi paradigme. C'est un croisement entre langage impératif (C), objet (C++), fonctionnel (Ocaml) et concurrent (Erlang). Le développement du langage RUST a commencé depuis 2009 par la fondation Mozila et aussi avec l’aide de la communauté des développeurs Rust qui sont très présent sur Github.

### Premier programme :

Markdown is a lightweight and easy-to-use syntax for styling your writing. It includes conventions for

```markdown
fn main() {
    println!("Bonjour tout le monde");
}
```

Nous avons crée un simple programme qui affiche le message (bonjour tout le monde).

### Déclaration des variables
 En rust les variables sont touts constant par défaut de déclaration.
 
 ## Un exemple :
 ```markdown
let i = 1;

i = 2; // ceci n’est pas valide  !
```

si on souhaite déclarer une variables mutable (modifiable) il faut utiliser le mot clé **mut** 

## Un exemple :

 ```markdown
let mut i = 1;

i = 2; // ceci est valide  !
```
## les Types :

  Comme tout les autres langages il existe plusieurs types de variables en rust,on a toujours des entiers, des flottants ,Strings etc.., La seule différence viendra de leur écriture. Par exemple, pour déclarer un entier de 32 bits, vous ferez :


 ```markdown
let i : i32 = 0;
// ou :
let i = 0i32;
```
Donc pour résumer, voici une petite liste des différents types de base disponibles : i8 (un entier signé de 8 bits),i16,i32,i64,u18(un entier non signé de 8bits),u16,u32,u64,f32(un flottant de 32bite),f64,String…

