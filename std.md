[**Base du langage**](https://uvsq21807686.github.io/RUST)-----[**Bibliothèques tierces**](https://uvsq21807686.github.io/RUST/trc)-----[**Outils de développement**](https://uvsq21807686.github.io/RUST/index2)-----[**Ressources d'apprentissages**](https://uvsq21807686.github.io/RUST/rsc)  

## Les bibliothèques standards :

La bibliothèque Rust Standard constitue la base du logiciel portable Rust, un ensemble d’abstractions partagées minimales et testées au combat pour l’ écosystème plus vaste de Rust . Il offre les types de base, comme Vec et  Option, définies par la bibliothèque des opérations sur des _primitives de langage_ ,  _macros standard_ , _E/S_ et _multithreading_.
_std_ est disponible par défaut pour toutes les caisses Rust, comme si chacune d'entre elles contenait une extern crate _std;_ importation à la racine de la caisse . Par conséquent, la bibliothèque standard est accessible dans les useinstructions via le chemin _std_, comme dans _use std::env_, ou dans les expressions via le chemin absolu _::std_, comme dans _::std::env::args_.


La bibliothèque Rust Standard est divisée en un certain nombre de modules ciblés, .  La documentation des modules comprend généralement une vue d'ensemble du module ainsi que des exemples.

### le contenu de la documentation de la bibliothèque standard 

Tout d'abord, la bibliothèque Rust Standard est divisée en un certain nombre de modules ciblés, qui sont tous répertoriés plus bas dans cette page . Ces modules constituent le socle sur lequel est forgée toute la rouille et portent des noms puissants tels que **std::sliceet** **std::cmp.** La documentation des modules comprend généralement une vue d'ensemble du module ainsi que des exemples. C'est un endroit intelligent pour commencer à vous familiariser avec la bibliothèque.

Deuxièmement, les méthodes implicites sur les types primitifs . Cela peut être source de confusion pour deux raisons:

Alors que les primitives sont implémentées par le compilateur, la bibliothèque standard implémente des méthodes directement sur les types primitifs (et c'est la seule bibliothèque qui le fait), qui sont documentées dans la section sur les primitives .

La bibliothèque standard exporte de nombreux modules portant le même nom que les types primitifs . Celles-ci définissent des éléments supplémentaires liés au type primitif, mais pas aux méthodes très importantes.
Ainsi, par exemple, il existe  le type primitif i32 qui répertorie toutes les méthodes pouvant être appelées sur des entiers 32 bits (très utile), et une page pour le module **std::i32 qui documente les valeurs constantes **MIN**et **MAX**(rarement utile).

Notez la documentation pour les primitives stret **[T]**(également appelé «slice»). De nombreuses méthodes appellent Stringet _Vec<T>_ sont en fait des appels à des méthodes sur stret _[T]_ respectivement, via des coercitions deref .

Troisièmement, la bibliothèque standard définit The Rust Prelude , une petite collection d’articles, principalement des traits, qui sont importés dans chaque module de chaque caisse. Les traits du prélude sont omniprésents, faisant de la documentation de prélude un bon point de départ pour se familiariser avec la bibliothèque.

Enfin, la bibliothèque standard exporte un certain nombre de macros standard et les répertorie sur cette page (techniquement, toutes les macros standard ne sont pas définies par la bibliothèque standard - certaines sont définies par le compilateur  . Comme le prélude, les macros standard sont importées par défaut dans toutes les caisses.





### Une visite de la bibliothèque standard Rust

Le reste de la documentation de caisse sert à mettre en évidence les caractéristiques notables de la bibliothèque Rust Standard.

**Conteneurs et collections**
Les modules optionet resultdéfinissent des types facultatifs et de traitement des erreurs, _Option<T>_et _Result<T,E>_. Le itermodule définit le trait itérateur de Rust Iterator, qui fonctionne avec la forboucle pour accéder aux collections.

La bibliothèque standard expose trois méthodes courantes pour gérer les régions de mémoire contiguës:

 •Vec<T>- Un vecteur alloué par tas qui est redimensionnable au moment de l'exécution.
 •[T; n]- Un tableau en ligne avec une taille fixe au moment de la compilation.
 •[T]- Une tranche de taille dynamique dans tout autre type de stockage contigu, affecté ou non au tas.
Les tranches ne peuvent être manipulées que par un type de pointeur , et en tant que telles, elles se déclinent en plusieurs saveurs telles que:

•&[T]- tranche partagée
•&mut [T]- tranche mutable
•Box<[T]>- tranche appartenant

**str**, une tranche de chaîne UTF-8, est un type primitif et la bibliothèque standard en définit de nombreuses méthodes. Rust **strs** sont généralement accessibles comme des références immuables: &str. Utilisez la propriété Stringpour créer et transformer des chaînes.

Pour la conversion en chaînes, utilisez la format!**macro**, et pour la conversion à partir de chaînes, utilisez le FromStrtrait.

Les données peuvent être partagées en les plaçant dans une zone ou un Rc type de références comptées. Si elles sont en outre contenues dans un Cellou RefCell, elles peuvent être mutées ou partagées. De même, en mode simultané, il est courant d’apparier une boîte comptée Arcavec une référence atomique , avec Mutexpour obtenir le même effet.

Le collectionsmodule définit les cartes, les ensembles, les listes chaînées et d’autres types de collections typiques, y compris le commun **HashMap<K, V>**.

### Abstractions de la plateforme et E / S

Outre les types de données de base, la bibliothèque standard vise principalement à résumer les différences entre les plates-formes communes, notamment les dérivés de Windows et Unix.

Les types courants de **E / S**, y compris les fichiers , TCP , UDP , sont définis dans les io, fset les netmodules.

Le threadmodule contient les abstractions de threading de Rust. sync contient d'autres types de mémoire partagée primitive, y compris atomicet mpsc, qui contient les types de canaux pour le transfert de messages.


### modules

 Premierement les méthodes implicites sur les modules  sont documentées ainsi quelques exemples :
 
```markdown
**alloc**	    API d'allocation de mémoire.
**ascii**	    Operations on ASCII strings and characters.
**boxed**	    Un type de pointeur pour l'allocation de tas.
**f32**	            Ce module fournit des constantes spécifiques à la mise en œuvre du f32type de données à virgule flottante.
**i8**	            Le type entier signé 8 bits.
**i16**	            Le type entier signé de 16 bits.
**i32**	            Le type entier signé 32 bits.
**i64**	            Le type entier signé 64 bits.
**i128**	    Le type entier signé de 128 bits.
**vec**	            Un type de tableau évolutif contigu avec un contenu alloué par tas, écrit Vec<T>

```

### les types primitifs

Deuxièmement, les méthodes implicites sur les types primitifs sont documentées ainsi quelques exemples :

```markdown
**array**	   Un tableau de taille fixe, désignée [T; N]par le type d'élément, Tet la taille constante non négative de la compilation, N.
**bool**	   Le type booléen.
**char**	   Un type de personnage.
**isize**	   Le type entier signé de la taille d'un pointeur.
**str**	           Tranches de ficelle.
**usize**	   Le type entier non signé de la taille d'un pointeur.

```

### les macros

 ```markdown

**assert**	       Assurez-vous qu'une expression booléenne est trueà l'exécution.
**assert_eq**	       Affirme que deux expressions sont égales (en utilisant PartialEq).
**cfg**		       Evaluation booléenne des drapeaux de configuration, à la compilation.
**column**	       Une macro qui se développe au numéro de colonne sur lequel elle a été appelée.
**compile_error**      Incidemment, la compilation échoue avec le message d’erreur donné lorsqu’elle est rencontrée.
**concat**	       Concatène les littéraux en une tranche de chaîne statique.  
**dbg**	               Une macro pour un débogage rapide et sale avec lequel vous pouvez inspecter la valeur d'une expression donnée
**debug_assert**       Assurez-vous qu'une expression booléenne est trueà l'exécution.
**debug_assert_eq**    Affirme que deux expressions sont égales.
**debug_assert_ne**    Affirme que deux expressions ne sont pas égales.
``` 

 
### Le prélude en Rust
Le prélude est la liste des éléments que Rust importé automatiquement dans chaque programme Rust. Elle est aussi petite que possible et se concentre sur des éléments, en particulier des traits, qui sont utilisés dans presque tous les programmes Rust.

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
  
**as**	       Le mot clé pour attribuer une valeur à un type.

**const**	    Le mot clé pour définir les constantes.

**crate**	    Le crate mot clé.

**enum**	     Pour définir des énumérations.

**extern**    Pour les connexions externes en code Rust.

**fn**	       Le mot clé pour définir les fonctions.

**for**	      Le for mot clé.

**if**	       Si déclarations et expressions.

**impl**	     Le mot clé définissant l'implémentation.

**let**	      Le mot clé de liaison variable.

**loop**	     Le mot-clé définissant la boucle.

**struct**    	Le mot-clé utilisé pour définir les structures.
