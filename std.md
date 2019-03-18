## les bibliotheque standards 

La bibliothèque Rust Standard constitue la base du logiciel portable Rust, un ensemble d’abstractions partagées minimales et testées au combat pour l’ écosystème plus vaste de Rust . Il offre les types de base, comme Vec<T>et Option<T>, définies par la bibliothèque des opérations sur des primitives de langage , macros standard , E / S et multithreading .
**std** est disponible par défaut pour toutes les caisses Rust, comme si chacune d'entre elles contenait une extern crate **std;** importation à la racine de la caisse . Par conséquent, la bibliothèque standard est accessible dans les useinstructions via le chemin **std**, comme dans **use std::env**, ou dans les expressions via le chemin absolu **::std**, comme dans **::std::env::args**.


La bibliothèque Rust Standard est divisée en un certain nombre de modules ciblés, .  La documentation des modules comprend généralement une vue d'ensemble du module ainsi que des exemples.
### modules

```markdown
**alloc**	API d'allocation de mémoire.
**ascii**	Operations on ASCII strings and characters.
**boxed**	Un type de pointeur pour l'allocation de tas.
**f32**	Ce module fournit des constantes spécifiques à la mise en œuvre du f32type de données à virgule flottante.
**i8**	Le type entier signé 8 bits.
**i16**	Le type entier signé de 16 bits.
**i32**	Le type entier signé 32 bits.
**i64**	Le type entier signé 64 bits.
**i128**	Le type entier signé de 128 bits.
**vec**	Un type de tableau évolutif contigu avec un contenu alloué par tas, écrit Vec<T>

```


Deuxièmement, les méthodes implicites sur les types primitifs sont documentées ici. Cela peut être source de confusion pour deux raisons:

Alors que les primitives sont implémentées par le compilateur, la bibliothèque standard implémente des méthodes directement sur les types de primitive (et c'est la seule bibliothèque qui le fait), qui sont documentées dans la section sur les primitives .
La bibliothèque standard exporte de nombreux modules portant le même nom que les types primitifs . Celles-ci définissent des éléments supplémentaires liés au type primitif, mais pas aux méthodes très importantes.
Ainsi, par exemple, il existe une page pour le type primitif i32 qui répertorie toutes les méthodes pouvant être appelées sur des entiers 32 bits (très utile), et une page pour le module std::i32 qui documente les valeurs constantes MINet MAX(rarement utile).

Notez la documentation pour les primitives stret [T](également appelé «slice»). De nombreuses méthodes appellent Stringet Vec<T>sont en fait des appels à des méthodes sur stret [T]respectivement, via des coercitions deref .

Troisièmement, la bibliothèque standard définit The Rust Prelude , une petite collection d’articles, principalement des traits, qui sont importés dans chaque module de chaque caisse. Les traits du prélude sont omniprésents, faisant de la documentation de prélude un bon point de départ pour se familiariser avec la bibliothèque.

Enfin, la bibliothèque standard exporte un certain nombre de macros standard et les répertorie sur cette page (techniquement, toutes les macros standard ne sont pas définies par la bibliothèque standard - certaines sont définies par le compilateur - mais elles sont documentées de la même manière). . Comme le prélude, les macros standard sont importées par défaut dans toutes les caisses.

