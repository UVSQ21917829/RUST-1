## les bibliotheque standards 

La bibliothèque Rust Standard constitue la base du logiciel portable Rust, un ensemble d’abstractions partagées minimales et testées au combat pour l’ écosystème plus vaste de Rust . Il offre les types de base, comme Vec<T>et Option<T>, définies par la bibliothèque des opérations sur des primitives de langage , macros standard , E / S et multithreading .
**std** est disponible par défaut pour toutes les caisses Rust, comme si chacune d'entre elles contenait une extern crate **std;** importation à la racine de la caisse . Par conséquent, la bibliothèque standard est accessible dans les useinstructions via le chemin **std**, comme dans **use std::env**, ou dans les expressions via le chemin absolu **::std**, comme dans **::std::env::args**.


La bibliothèque Rust Standard est divisée en un certain nombre de modules ciblés, .  La documentation des modules comprend généralement une vue d'ensemble du module ainsi que des exemples.
### modules
 Premierement les méthodes implicites sur les modules  sont documentées ainsi quelques exemples :
```markdown
**alloc**	  API d'allocation de mémoire.
**ascii**	  Operations on ASCII strings and characters.
**boxed**	  Un type de pointeur pour l'allocation de tas.
**f32**	    Ce module fournit des constantes spécifiques à la mise en œuvre du f32type de données à virgule flottante.
**i8**	    Le type entier signé 8 bits.
**i16**	    Le type entier signé de 16 bits.
**i32**	    Le type entier signé 32 bits.
**i64**	    Le type entier signé 64 bits.
**i128**	  Le type entier signé de 128 bits.
**vec**	    Un type de tableau évolutif contigu avec un contenu alloué par tas, écrit Vec<T>

```

### les types primitifs
Deuxièmement, les méthodes implicites sur les types primitifs sont documentées ainsi quelques exemples :

```markdown
**array**	Un tableau de taille fixe, désignée [T; N]par le type d'élément, Tet la taille constante non négative de la compilation, N.
**bool**	Le type booléen.
**char**	Un type de personnage.
**isize**	Le type entier signé de la taille d'un pointeur.
**str**	  Tranches de ficelle.
**usize**	Le type entier non signé de la taille d'un pointeur.

```

### les macros
 ```markdown
**assert**	        Assurez-vous qu'une expression booléenne est trueà l'exécution.
**assert_eq**	      Affirme que deux expressions sont égales (en utilisant PartialEq).
**cfg**		          Evaluation booléenne des drapeaux de configuration, à la compilation.
**column**		      Une macro qui se développe au numéro de colonne sur lequel elle a été appelée.
**compile_error**   Incidemment, la compilation échoue avec le message d’erreur donné lorsqu’elle est rencontrée.
**concat**	        Concatène les littéraux en une tranche de chaîne statique.
**dbg**	            Une macro pour un débogage rapide et sale avec lequel vous pouvez inspecter la valeur d'une expression donnée
**debug_assert**    Assurez-vous qu'une expression booléenne est trueà l'exécution.
**debug_assert_eq**	Affirme que deux expressions sont égales.
**debug_assert_ne**	Affirme que deux expressions ne sont pas égales.
 
 ```
 
### The Rust Prelude
Le prélude est la liste des éléments que Rust importe automatiquement dans chaque programme Rust. Elle est aussi petite que possible et se concentre sur des éléments, en particulier des traits, qui sont utilisés dans presque tous les programmes Rust.

Sur le plan technique, les inserts Rust

 ```markdown
 extern crate std;
  ```
  
  dans la racine de la caisse de chaque caisse, et
  
  ```markdown
 use std::prelude::v1::*;
  ```
  
  dans chaque module.
  ### Mots clés
  
**as**	    Le mot clé pour attribuer une valeur à un type.

**const**	  Le mot clé pour définir les constantes.

**crate**	  Le crate mot clé.

**enum**	  Pour définir des énumérations.

**extern**  Pour les connexions externes en code Rust.

**fn**	    Le mot clé pour définir les fonctions.

**for**	    Le for mot clé.

**if**	    Si déclarations et expressions.

**impl**	  Le mot clé définissant l'implémentation.

**let**	    Le mot clé de liaison variable.

**loop**	  Le mot-clé définissant la boucle.

**struct**	Le mot-clé utilisé pour définir les structures.
