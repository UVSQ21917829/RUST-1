[**Base du langage**](https://UVSQ21917829.github.io/RUST-1)----- [**Bibliothèques standards**](https://UVSQ21917829.github.io/RUST-1/std) -----[**Bibliothèques tierces**](https://UVSQ21917829.github.io/RUST-1/trc)-----[**Outils de développement**](https://UVSQ21917829.github.io/RUST-1/index2)-----[**Ressources d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/rsc)-----[**Exercices d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/exo)   


# Les concepts avancés
## Les pointeurs intelligents (Smart pointer) 

Dans Rust, les pointeurs intelligents ne sont pas seulement des pointeurs mais également une structure de données. Ce concept de pointeurs intelligents n'est pas seulement dans le language Rust, il existe aussi dans d'autres languages et ils sont  originaires de C ++. Dans Rust, les pointeurs intelligents définis dans la bibliothèque standard offrent des fonctionnalités au-delà de celles fournies par les références.
### Box
Box est le pointeur intelligent le plus simple qui permet de stocker des données sur le tas (Heap). Nous utilisons le type de Box pour fournir la taille du type afin que le compilateur sache combien de mémoire doit être allouée.
La pile (Stack) contient le pointeur sur les données du tas. Un Box n'a pas de surcharge de performances, autre que le stockage de leurs données sur le tas.

#### Utilisation de Box \<T> pour stocker les données sur le tas

Cet exemple montre comment utiliser un Box pour stocker une donnée  sur le tas:

```
fn main()  
{  
  let a = Box :: new(1);  
  print!("La valeur de a est : {}",a);  // Affiche: La valeur de a est : 1
}   

```
#### Cons List et Types récursifs avec Box

Cons List est une structure de données qui est utilisée pour construire une nouvelle paire à partir des deux arguments, et cette paire est connue sous le nom de liste.
Cons List contient deux éléments, l'élément actuel et le dernier élément. Le dernier élément de la liste est Nil car Nil ne contient pas l'élément suivant.

Exemple d'utilisation: 

Nous pouvons utiliser le pointeur Box <T> car le compilateur sait combien d'espace le pointeur Box <T> nécessite. La taille du pointeur Box <T> ne changera pas pendant l'exécution d'un programme. Le pointeur Box <T> pointe vers la valeur List qui sera stockée sur le tas plutôt que dans la variante cons.
  
```
enum List {
   Cons(i32, Box),
   Nil,
}

use List::{Cons, Nil};

fn main() {
let list = Cons(1,
                  Box::new(Cons(2,
                                Box::new(Cons(3,
                                              Box::new(Nil))))));
}
```
### Deref Trait

Le trait Deref nous permet de personnaliser le comportement de déréférencement d'un opérateur. Le trait Deref est défini dans la bibliothèque standard qui est utilisée pour implémenter la méthode nommée deref.

##### Exemple d'implementation:

````
struct MonBox<T>  
{  
  a : T,  
}  
use :: std::ops::Deref; 

impl<T> Deref for MonBox<T>  
{  
  type Target = T;  
  fn deref(&self) ->&T  
  {  
    &self.a  
  }  
} 

````
##### Explication

- Le trait Deref est implémenté sur le type MonBox.
- Le trait Deref implémente la méthode deref() et la méthode deref() renvoie la référence de la variable 'a'.
- Le type Target = T; est un type associé pour un trait Deref. Le type associé est utilisé pour déclarer le paramètre de type générique.
### Drop trait 

Le trait Drop est un pointeur intelligent qui nous permet de personnaliser ce qui se passe avec la valeur une fois qu'elle est hors de Scope. Le trait drop est utilisé pour implémenter la méthode drop() qui prend une référence mutable à Self.


##### Exemple d'implementation:
```
struct Exemple{  
  a : String,  
}  
  
impl Drop for Exemple  
{  
  fn drop(&mut self)  
  {  
    println!("destruction de l'instance de Exemple : {}", self.a);  
  }  
}  
  
fn main()  
{  
  let var1 = Exemple{a : String::from("Bonjour")};  
  drop(var1);  
  let var2 = Exemple{a: String::from("Tous le monde")};  
  println!("Ceration de l'instance de Exemple ");  
}  

```
l'instance var1 est détruite en passant l'instance var1 comme argument dans la fonction drop(var1)


### Rc 
  Pour activer la propriété multiple, Rust a un type appelé Rc <T>, qui est une abréviation pour le comptage des références. Le type Rc <T> assure le suivi du nombre de références à une valeur qui détermine si une valeur est toujours utilisée ou non. S'il n'y a aucune référence à une valeur, la valeur peut être nettoyée sans qu'aucune référence ne devienne invalide.
  Nous utilisons le type Rc <T> lorsque nous voulons allouer des données sur le tas à plusieurs parties de notre programme à lire et nous ne pouvons pas déterminer au moment de la compilation quelle partie finira d'utiliser les données en dernier. Si nous savions quelle partie se terminerait en dernier, nous pourrions simplement faire de cette partie le propriétaire des données, et les règles de propriété normales appliquées au moment de la compilation prendraient effet.
  
##### Exemple d'implementation:
```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

### RefCell et le modèle de mutabilité intérieure
La mutabilité intérieure est un modèle de conception dans Rust qui vous permet de muter des données même lorsqu'il existe des références immuables à ces données. Pour muter les données, le modèle utilise du code non sécurisé dans une structure de données pour contourner les règles habituelles de Rust qui régissent la mutation et l'emprunt. Le code dangereux impliqué est ensuite enveloppé dans une API sûre, et le type externe est toujours immuable.
Du coup les RefCell sont utiles pour garder un accès mutable sur un objet. Le "borrowing" est alors vérifié au runtime plutôt qu'à la compilation.
```
 use std::cell::RefCell;  
fn main()  
{  
  let a = RefCell::new(15);  
  let b = a.borrow();  
  let c = a.borrow();  
  println!("la valeur de b est : {}",b);  
  println!("la valeur de c est : {}",c);  
}  
```
## Propiété (Ownership)

La propriété est une caractéristique centrale de Rust et parmit l'un de ses points forts. Un système de propriété gère la mémoire avec un ensemble de règles que le compilateur vérifie au moment de la compilation.

En Rust chaque valeur a une variable nommée propriétaire (Owner) de la valeur. Chaque donnée stockée dans Rust sera associé à un propriétaire. Il faut savoir que :

- Chaque donnée ne peut avoir qu'un seul propriétaire à la fois.

- Deux variables ne peuvent pas pointer vers le même emplacement mémoire. 

Exemple:
```markdown 
fn main() 
{ 
    let matier_p : String = String::from("RUST"); 
    let matier_d : String = matier_p ; // affecter matier_p à matier_d
    let matier_t : String = matier_p; // Erreur!! parce que  la ressource a été déplacée.
}

```

Dans le cas des types primitifs, le contenu d'une variable est copié dans une autre. Il n'y a donc pas de transfert de propriété. En effet, une variable primitive a besoin de moins de ressources qu'un objet. 
## Généricité

La généricité permet de gérer efficacement la duplication des concepts et de réduire les répétitions au sein du code dans de nombreux cas. Elle à généraliser les types et des fonctionnalités à des cas plus larges. Etre générique nécessite de bien savoir spécifier sur quels types un type générique est réellement considéré comme valide. L'utilisation la plus simple et la plus courante des génériques concerne les paramètres de type.