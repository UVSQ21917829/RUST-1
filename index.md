[**Principales bibliothèques du langage **](https://uvsq21807686.github.io/RUST/std)[**Principales bibliothèques tierces **](https://uvsq21807686.github.io/RUST/trc) ------------ [**Outils de developpement**](https://uvsq21807686.github.io/RUST/index2) ------------  [**Ressources d’apprentissages**](https://uvsq21807686.github.io/RUST/rsc)  


## Bienvenue sur ma page 
## Base du langage

## 1.1 introduction

 Rust est un langage de programmation système moderne mettant l'accent sur la sécurité, la vélocité et la concurrence, compilé et multi paradigme. C'est un croisement entre langage impératif (C), objet (C++), fonctionnel (Ocaml) et concurrent (Erlang). Le développement du langage RUST a commencé depuis 2009 par la fondation Mozila et aussi avec l’aide de la communauté des développeurs Rust qui sont très présent sur Github.

### Premier programme :

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

## Incrémentation : 
pour incrémenter il faudra utiliser la syntaxe : 
```markdown
          i += 1;
```

### Les Conditions : 

On utilise les mêmes mot clé que java/c++/c :

```markdown

let moyenne : i32 = 17;
if moyenne >= 10 {
    println!("admis !");
} else {
    println!("ajourné !");
}

```
Pattern matching : c’est une autre façon de reformuler les conditions, d’une manière on pourra comparer les expressions ainsi :
```markdown

let marque= "206";
let mut i = 1;
match marque {
    "Partner " => {
        println!("Peugeot");
    }
    "AMG" => {
        println!("Mercedes Benz");
    }
    "Clio" => {
        println!("Renault ");
    }
    "Yaris" => {
        println!("Toyota");
    }
    _ => {
        println!("je ne connais pas cette marque...");
    }
}


```

Le _ est utilisé pour le cas ou le résultat ne signifie aucun des expressions précédentes, c’est l’équivalant de default  d’un Switch en Java  

### Les boucles :

Les boucles en Rust sont comme les autres langage, elle continue à exécuter tant que la condition est toujours vérifiés 
## La boucle while :

## Exemple :

```markdown
        
let mut i : i32 = 0;
while i < 10 {
    println!("i inferieur a 10 !");
    i += 1;
}

```

Ici le programme affichera a chaque fois « i inferieur a 10 » tant que le i inferieur (la condition i<10) 

•	Vous noterez encore une fois qu'il n'y a pas de parenthèses autour de la condition !


•	Tout comme pour les conditions, les accolades sont encore une fois obligatoires 

## Loop :
Le boucle loop est un infinie ,c’est  l’équivalent de « while true » 
```markdown

while true {
    
     //...
}
```
Sa condition d’arrêt se fait a la fin du programme en ajoutant les mots clé break ou return .

## Exemple 1 :
```markdown

let mut i : i32 = 0;
loop {
    println!("i inferieur a 10 !");
    i += 1;
   if i > 10 {
      break;

   }
}


```
## Exemple 2 :
```markdown

let mut i : i32 = 0;
loop {
    println!("i inferieur a 10 !");
    i += 1;
   if i > 10 {
      return;

   }
}

```
### For :

La boucle for consiste à exécuter le programme en suivant la condition de départ 
## Exemple :

```markdown

for i in 0..10 {
    println!("la valeur de i est  : {}", i);
}

```
 Ce programme affiche a chaque fois la valeur de i de 0 a 9

### Les fonctions :

 Pour utiliser une fonction il faut suivre la syntaxe suivante 
```markdown 
 
fn nom_fonction(les variable d’entrer)->type_sortie
```
## exemple :
```markdown
fn addition(nb1: i32, nb2: i32) -> i32;


```

ici on a déclarer une fonction nommé addition qui prend en paramètre deux variable de type i32 et qui return un i32 

## exemple d’utilisation :
```markdown
fn main() {
    println!("1 + 2 = {}", addition(1, 2));
}

fn addition(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}
```

Ce programme affiche 1 + 2 =3


 et pour plus de decumentations veuillez consulter ces **liens suivants**	:
   
   •le site internet [**rust-lang.org**](https://www.rust-lang.org/)
   
   •la [**decumentation**](https://doc.rust-lang.org/stable/std/)
   
   •le [**depot github**](https://github.com/rust-lang/rust)
   
   •le [**rustbook**](https://doc.rust-lang.org/stable/book/)
   
   en conclusion **Rust** à été concu comme langage systéme pour remplacer **C++** avec une gestion de la mémouire 
   plus sure ,un point qu'on ne lui disputera pas 
    

 
 
