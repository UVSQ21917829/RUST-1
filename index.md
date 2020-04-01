[**Concepts avancés**](https://UVSQ21917829.github.io/RUST-1/concept)----- [**Bibliothèques standards**](https://UVSQ21917829.github.io/RUST-1/std) -----[**Bibliothèques tierces**](https://UVSQ21917829.github.io/RUST-1/trc)-----[**Outils de développement**](https://UVSQ21917829.github.io/RUST-1/index2)-----[**Ressources d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/rsc)-----[**Exercices d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/exo)   


 
## Base du langage

## 1.1 introduction

 Rust est un langage de programmation système moderne mettant l'accent sur la sécurité, la vélocité et la concurrence, compilé et multi paradigme. C'est un croisement entre langage impératif (C), objet (C++), fonctionnel (Ocaml) et concurrent (Erlang). Le développement du langage RUST a commencé depuis 2009 par la fondation Mozila et aussi avec l’aide de la communauté des développeurs Rust qui sont très présent sur Github.

### Premier programme :

```markdown
fn main() {
    println!("Bonjour tout le monde");
}
```

Nous avons créé un simple programme qui affiche le message (bonjour tout le monde).

### Déclaration des variables
 En rust les variables sont toutes constantes par défaut de déclaration et elles sont déclarées par **let**.
 
 
### Exemple :  
 ```markdown
let i = 1;

i = 2; // ceci n’est pas valide  !
```

Si on souhaite déclarer une variable mutable (modifiable) il faut utiliser le déclarateur  **mut** 

## Exemple :

 ```markdown
let mut i = 1;

i = 2; // ceci est valide  !
```
## les Types prémitifs:

  Comme touts les autres langages il existe plusieurs types de variables en rust,on a toujours des entiers, des flottants ,Strings etc.., La seule différence viendra de leur écriture. 
### Entier
Un nombre entier est un nombre que l’on peut écrire sans virgule. En Rust les types d’entiers signés commencent par i et non signés par u. 

 ```markdown
let i : i32 = 0;
// ou :
let i = 0i32;
```
### Les flottants

Un flottant est un nombre avec la virgule. Les types flottantes de Rust sont f32 et f64, qui sont respectivement de 32 bits et 64 bits. Le type par défaut est f64.
````
fn main() {

    let x = 0.0; // Variable de type float 64 bit: type inplicite
    let xx: f64 = 0.0; // Variable de type float 64 bit: type explicite 
}
````

### Bool

Un booléen standard. Peut être vrai ou faux

```markdown
let t = true;
let f = false;
```
### Char
Un caractère de 4 octets.

```markdown
let a = 'a';
let b = 'b';
let keyboard = '⌨';
```
## Types composés
Les types composés peuvent regrouper plusieurs valeurs en un seul type

### Tableau
Un tableau est une taille fixe, une collection d'éléments de même type.

Il est déclaré comme:
```markdown
let name: [type; size] = [v1, v2, v3, v4];
```

```markdown
let array: [i32; 5] = [0, 1, 2, 3, 4];

println!("le premier element est: {}", array[0]);

let mut compteur = 0;
for x in array.iter(){
    println!("l'element a l'index {} est {}", compteur, x);
    compteur += 1;
}
```
### Tuple

Les tuples sont des séquences finies. Tout d'abord ils sont fini, ils ont une taille, un nombre fixe d'éléments. Ils peuvent contenir plusieurs types différents. Cela contraste avec un tableau, qui ne peut contenir que des éléments du même type. Enfin, il s’agit de séquences, ce qui signifie qu’elles ont un ordre, et surtout qu’on peut y accéder par index (bien que de manière différente de celle des tableaux).

```markdown
let tuple = ("Bonjour", 42, "Monde", [3,6,9]);

println!("premier element  est {}", tuple.0);
println!("deuxieme element est  {}", tuple.1);
println!("troisieme element est {}", tuple.2);
let mut compteur = 0;
for x in &tuple.3 {
    println!("Element {} du quatriere element est {}", compteur, x);
    compteur += 1;
}
```
### Slices

Slice est un pointeur sur un bloc de mémoire. La slice peut être utilisée pour accéder à des parties de données stockées dans des blocs de mémoire contigus. Elle peut être utilisée avec des structures de données comme des tableaux, des vecteurs et des strings. Les slices sont des pointeurs vers les données réelles. La taille d’une slice est déterminée lors de l’exécution.

Par exemple, si nous voulions prendre les 3 premiers éléments de notre tableau, cela ressemblerait à ceci:

```markdown
let array: [i32; 5] = [0, 1, 2, 3, 4];

let t = &array[0..3]; 

for x in t {
	println!("x est {}", x):
}
```
le **&** fait référence à la mémoire réelle de ce qu'un tableau "pointe" sur.

### Str
 
 Str est une "slice de chaîne" et constitue le type de chaîne le plus primitif.

```markdown

let str = "Bonjour je suis une chaine de caractere ";

println!("la valeur de notre variable est : {}", str);

```

### Struct

Un struct type est un produit hétérogène d'autres types, appelés champs du type.
Structs sont un moyen de créer des types de données plus complexes. Par exemple, si nous faisions des calculs impliquant des coordonnées dans un espace 2D, x et y.Cela nous permet de combiner ces deux en un seul type de données unifié avec x et y comme étiquettes de champ:

```markdown
struct  Point {
     x : i32 ,
     y : i32 ,
}

fn  main () {
     let  origine  =  Point { x : 0 , y : 0 }; 

    println! ( "L'origine est à ({}, {})" , origine . X , origine . Y );
}
```

Donc pour résumer, voici une petite liste des différents types de base disponibles : i8 (un entier signé de 8 bits),i16,i32,i64,u18(un entier non signé de 8bits),u16,u32,u64,f32(un flottant de 32bits),f64,St, reference, tuple, tableau, slice, struct.

## Reference


Il y a un deuxième type de référence: &mut T. Une "référence mutable" vous permet de muter la ressource que vous empruntez. Par exemple:
```markdown
let  mut  x  =  5 ; 
{ let y = & mut x ;
    * y + = 1 ; 
} println ! ( "{}" , x );
```
Cela va afficher 6. Nous faisons y référence à x.

En Gros on a : une ou plusieurs références ( &T) à une ressource et exactement une référence mutable ( &mut T).

## Incrémentation : 
pour incrémenter il faudra utiliser la syntaxe : 

```markdown
          i += 1;
```

## Les Conditions : 

On utilise les mêmes mot clé que java/c++/c :

```markdown

let moyenne : i32 = 17;
if moyenne >= 10 {
    println!("admis !");
} else {
    println!("ajourné !");
}

```
**Pattern matching** : c’est une autre façon de reformuler les conditions, d’une manière on pourra comparer les expressions ainsi :
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

## Les boucles :

Les boucles en Rust sont comme dans les autres langage, elles continuent à exécuter tant que la condition est toujours vérifié 
### La boucle while :

**Exemple :**

```markdown
        
let mut i : i32 = 0;
while i < 10 {
    println!("i inferieur a 10 !");
    i += 1;
}

```

Ici le programme affichera a chaque fois « i inferieur a 10 » tant que **i** est inferieur (la condition i<10) 

•	Vous noterez encore une fois qu'il n'y a pas de parenthèses autour de la condition !


•	Tout comme pour les conditions, les accolades sont obligatoires. 

### Loop :
loop est une boucle infinie (toujours vrai). C’est  l’équivalent de « while true ». 
```markdown

while true {
    
     //...
}
```
Sa condition d’arrêt se fait à la fin du programme en ajoutant les mots clé **break** ou **return**.

**Exemple 1 :**
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
**Exemple 2 :**
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

**For** peut être utilisé pour itérer sur un Iterateur. L’une des façons les plus simples de créer un itérateur est d’utiliser la notation de l’intervale a..b. Cela donne des valeurs de a (inclus) à b (exclus) par pas de 1.

**Exemple :**

```markdown

for i in 0..10 {
    println!("la valeur de i est  : {}", i);
}

```
 Ce programme affiche à chaque fois la valeur de i de 0 à 9

### Les fonctions :
Les fonction dans Rust commencent par fn et elles ont un ensemble de parenthèses après le nom de la fonction. Les accolades indiquent au compilateur où commence et se termine le corps de la fonction. Les fonctions peuvent aussi renvoyer des valeurs. Nous ne nommons pas les valeurs de retour, mais nous déclarons leur type après une flèche (->).

Pour utiliser une fonction, il faut suivre la syntaxe suivante:
 
```markdown 
 
fn nom_fonction(les variable d’entrer)->type_sortie
```
**Exemple :**
```markdown
fn addition(nb1: i32, nb2: i32) -> i32;


```

Ici, on a déclaré une fonction nommée **addition** qui prend en paramètre deux variables de type **i32** et qui retourne une valeur de type **i32** 

**Exemple d’utilisation :**
```markdown

fn main() {
    println!("1 + 2 = {}", addition(1, 2));
}

fn addition(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}
```

Ce programme affiche 1 + 2 =3



 Et pour plus de documentations veuillez consulter ces **liens suivants**	:
   
   •le site internet [**rust-lang.org**](https://www.rust-lang.org/)
  
   •le [**depot github**](https://github.com/rust-lang/rust)
   
   •le [**rustbook**](https://doc.rust-lang.org/stable/book/)
   
   En conclusion **Rust** à été concu comme langage systéme pour remplacer **C++** avec une gestion de la mémoire 
   plus sure ,un point qu'on ne lui disputera pas .
    

 
 
