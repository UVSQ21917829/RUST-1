[**Bibliothèques standards**](https://uvsq21807686.github.io/RUST/std)-----[**Bibliothèques tierces**](https://uvsq21807686.github.io/RUST/trc)-----[**Outils de développement**](https://uvsq21807686.github.io/RUST/index2)-----[**Ressources d'apprentissages**](https://uvsq21807686.github.io/RUST/rsc)   


 
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
 En rust les variables sont toutes constantes par défaut de déclaration.
 
 
### Exemple :  
 ```markdown
let i = 1;

i = 2; // ceci n’est pas valide  !
```

si on souhaite déclarer une variable mutable (modifiable) il faut utiliser le mot clé **mut** 

## Exemple :

 ```markdown
let mut i = 1;

i = 2; // ceci est valide  !
```
## les Types :

  Comme touts les autres langages il existe plusieurs types de variables en rust,on a toujours des entiers, des flottants ,Strings etc.., La seule différence viendra de leur écriture. Par exemple, pour déclarer un entier de 32 bits, vous ferez :


 ```markdown
let i : i32 = 0;
// ou :
let i = 0i32;
```
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

### tableau
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
### Slices

Une tranche est une taille dynamique, "tranche" dans une collection d'éléments.

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
 
 Str est une "tranche de chaîne" et constitue le type de chaîne le plus primitif.

```markdown
let str = "Bonjour je suis une chaine de caractere ";

println!("la valeur de notre variable est : {}", str);

```


### tuple

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
### Reference


Il y a un deuxième type de référence: &mut T. Une "référence mutable" vous permet de muter la ressource que vous empruntez. Par exemple:
```markdown
let  mut  x  =  5 ; 
{ let y = & mut x ;
    * y + = 1 ; 
} println ! ( "{}" , x );
```
Cela va afficher 6. Nous faisons y référence à x.

En Gros on a : une ou plusieurs références ( &T) à une ressource et exactement une référence mutable ( &mut T).

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

## Gestion de la mémoire:

 Rust propose une nouvelle façon de gérer la mémoire, qui se veut sûre et garantie à la compilation, ce qui permet d’en limiter l’impact sur les performances à l’exécution. Autant vous prévenir tout de suite : la courbe d’apprentissage de Rust est donc plus lente que dans d’autres langages mais largement compensée en qualité et fiabilité des programmes produits. Concrètement, vous allez transpirer au début mais serez fiers de la qualité de vos productions.
 
 **Ownership** : Rust contrôle la gestion de la mémoire à travers des règles strictes,lorsqu'un cadre de pile est quitté, ses allocations locales sont toutes libérées et ses références aux zones sont supprimées.

 Rust est muni d'un système « d'appartenance » qui permet d'écarter les conflits les plus communs lorsqu'une ressource est utilisée à plusieurs endroits.
Bien que ce dernier soit très pratique, il demande d'avoir une certaine rigueur quant à la déclaration de nos ressources, sans quoi vous risqueriez de vous attirer les foudres du compilateur.
Rust possède une syntaxe rigide et un typage fort permettant de s’assurer qu’il n’y ait aucune fuite mémoire après compilation. Au moment de la compilation, le code est analysé de manière à ce que le langage indique directement les problèmes rencontrés dans la gestion de la mémoire. Le compilateur de Rust contraint ainsi le programmeur à corriger ses erreurs avant toute exécution du programme. Il est donc impossible d’avoir un dépassement de mémoire tampon.


## smart pointers
Dans Rust, les pointeurs intelligents ne sont pas seulement des pointeurs, mais également une structure de données. Ils sont également disponibles dans de nombreuses autres langues, mais leur origine est en c ++. Les pointeurs intelligents sont généralement implémentés via des structures.
Le motif de pointeur intelligent est un motif de conception général utilisé fréquemment dans Rust . De nombreuses bibliothèques ont leurs propres pointeurs intelligents et vous pouvez même écrire les vôtres. Nous allons couvrir les pointeurs intelligents les plus courants dans la bibliothèque standard:

•Box<T> pour allouer des valeurs sur le tas.
	
•Rc<T>, un type de comptage de référence qui permet la propriété multiple.
	
•Ref<T>et RefMut<T>, accessible via RefCell<T>, un type qui applique les règles d'emprunt au moment de l'exécution au lieu du moment de la compilation


Voici la syntaxe d'utilisation Box<T>
	
	
```markdown
	fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

 Et pour plus de documentations veuillez consulter ces **liens suivants**	:
   
   •le site internet [**rust-lang.org**](https://www.rust-lang.org/)
  
   •le [**depot github**](https://github.com/rust-lang/rust)
   
   •le [**rustbook**](https://doc.rust-lang.org/stable/book/)
   
   En conclusion **Rust** à été concu comme langage systéme pour remplacer **C++** avec une gestion de la mémoire 
   plus sure ,un point qu'on ne lui disputera pas .
    

 
 
