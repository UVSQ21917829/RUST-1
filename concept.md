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
