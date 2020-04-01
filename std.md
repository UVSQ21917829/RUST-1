[**Base du langage**](https://uvsq21807686.github.io/RUST-1)-----[**Concepts avancés**](https://uvsq21807686.github.io/RUST-1/concept)-----[**Bibliothèques tierces**](https://uvsq21807686.github.io/RUST-1/trc)-----[**Outils de développement**](https://uvsq21807686.github.io/RUST-1/index2)-----[**Ressources d'apprentissages**](https://uvsq21807686.github.io/RUST-1/rsc)-----[**Exercices d'apprentissages**](https://uvsq21807686.github.io/RUST-1/exo)

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
  ## Les types de la bibliothèque standard

### Les vecteurs


  
   
  Les vecteurs sont des tableaux redimensionnables. Tout comme les slices, leur taille n'est pas connue à la compilation mais ils peuvent être agrandis ou tronqués au cours de l'exécution. Un vecteur est représenté par trois (3) mots : un pointeur sur la ressource, sa taille et sa capacité. La capacité indique la quantité de mémoire réservée au vecteur. La taille peut augmenter à volonté, tant qu'elle est inférieure à la capacité. Lorsqu'il est nécessaire de franchir cette limite, le vecteur est réalloué avec une capacité plus importante
  
  ```markdown
let v: Vec<i32> = vec![];

let v = vec![1, 2, 3, 4, 5];

let v = vec![0; 10]; // ten zeroes
  
 ```
  
### L’énumération Option
 
 Le type Option représente une valeur facultative: every Option est soit Someet contient une valeur, soit None, et ne contient pas. Optionles types sont très courants dans Rust code
 
  ```markdown
 let  msg  =  Some ( "comment va" );

// Prend une référence à la chaîne contenue 
si  let  Some ( ref  m ) =  msg {
     println ! ( "{}" , * m );
}

// Supprime la chaîne contenue, en détruisant l'option, 
laissez  unwrapped_msg  =  msg . unwrap_or ( "message par défaut" ); 
  
```


### L'énumération Result

Result<T, E> est le type utilisé pour renvoyer et propager des erreurs. C'est une énumération avec les variantes,, Ok(T)représentant le succès et contenant une valeur, et Err(E) représentant l'erreur et contenant une valeur d'erreur.

  ```markdown
enum  Résultat < T , E > {
    Ok ( T ),
    Err ( E ),
} 
```
### Lire et écrire:
  
  Parce qu'ils sont des traits Readet Write sont implémentés par un certain nombre d'autres types, vous pouvez également les implémenter pour vos types. En tant que tel, vous verrez quelques types d’E / S différents dans la documentation de ce module: Files, TcpStreams et parfois même Vec<T>s. Par exemple, Read ajoute une read méthode, que nous pouvons utiliser sur Files:
 
   ```markdown
   
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    f.read(&mut buffer)?;

    println!("The bytes: {:?}", buffer);
    Ok(())
}
 
 
 ```
  
### La structure HashMap
Là où les vecteurs stockent leurs valeurs en utilisant un index entier, les **HashMaps** stockent leurs valeurs en utilisant des clés. Les clés d'une  HashMap peuvent être des booléens, des chaînes de caractères ou n'importe quel autre type qui implémente les traits Eq et Hash. Nous y reviendrons dans la section suivante.

Tout comme les vecteurs, les HashMap sont redimensionnables mais peuvent également se tronquer elles-mêmes lorsqu'elles atteignent la limite de leur capacité. Vous pouvez créer une HashMap avec une capacité donnée en utilisant HashMap::with_capacity(uint), ou utiliser  HashMap::new() pour récupérer une instance avec une capacité initiale par défaut (recommandé).

Une HashMapliste d'éléments fixes peut être initialisée à partir d'un tableau:
 ```markdown
use std::collections::HashMap;
fn main() {
          let timber_resources: HashMap<&str, i32> =
          [("Norway", 100),
          ("Denmark", 50),
          ("Iceland", 10)]
          .iter().cloned().collect();
          // use the values stored in map
          }
   ```
   

### thread


   
   Un programme Rust en cours d’exécution consiste en un ensemble de threads de système d’exploitation natifs, chacun avec sa propre pile et son propre état. Les threads peuvent être nommés et fournissent une prise en charge intégrée pour la synchronisation de bas niveau.
   
   ```markdown
   use std::thread;

thread::spawn(move || {
    // des instructions
                      }
```



### dbg
   
   une macro pour le débogage rapide et sale avec lequel vous pouvez inspecter la valeur d'une expression donnée. Un exemple:
   
 ```markdown
 let a = 2;
 let b = dbg!(a * 2) + 1;
   //      ^-- prints: [src/main.rs:2] a * 2 = 4
 assert_eq!(b, 5);
 
    ```
   
   
### panic!
   
   Prise en charge de la panique dans la bibliothèque standard.
   **implementation** :Si le thread principal panique, il mettra fin à tous vos threads et à votre programme avec du code 101.
   ```markdown
   fn division(dividend: i32, divisor: i32) -> i32 {
              if divisor == 0 {
                              // La division par zéro fait planter le thread courant.
                              panic!("division by zero");
                              } else {
                                     dividend / divisor
                                     }
                                                    }

```
### ascii
Opérations sur les chaînes et les caractères ASCII.

La plupart des opérations sur les chaînes dans Rust agissent sur les chaînes UTF-8. Cependant, il est parfois plus logique de ne considérer que le jeu de caractères ASCII pour une opération spécifique.
**AsciiExt**
```markdown
use std::ascii::AsciiExt;

assert_eq!(AsciiExt::to_ascii_uppercase("café"), "CAFÉ");
assert_eq!(AsciiExt::to_ascii_uppercase("café"), "CAFé");
```


### net
Primitives de mise en réseau pour la communication TCP / UDP.

Ce module fournit une fonctionnalité réseau pour les protocoles de contrôle de transmission et de datagramme utilisateur, ainsi que des types pour les adresses IP et de socket.

### clone
Le Clone trait pour les types qui ne peuvent pas être "copiés implicitement".
Exemple d'utilisation basique:
```markdown
let  s  =  String :: new (); // Le type de chaîne implémente Clone 
let  copy  =  s . clone (); // afin que nous puissions le cloner 
```

### time

Quantification temporelle:

 ```markdown
 
 use std::time::Duration;

let five_seconds = Duration::new(5, 0);
// les deux déclarations sont équivalentes
assert_eq!(Duration::new(5, 0), Duration::from_secs(5));
  ```
  
### path

  
  Manipulation de chemins multi-plateformes.
Ce module fournit deux types, PathBufet Path(apparenté à String et str), pour travailler avec des chemins abstraits. Ces types sont des wrappers minces autour OsString et OsStrrespectivement, ce qui signifie qu'ils travaillent directement sur les chaînes en fonction de la syntaxe de chemin de la plateforme locale.
```markdown
use std::path::Path;
use std::ffi::OsStr;

let path = Path::new("/tmp/foo/bar.txt");

let parent = path.parent();
assert_eq!(parent, Some(Path::new("/tmp/foo")));

let file_stem = path.file_stem();
assert_eq!(file_stem, Some(OsStr::new("bar")));

let extension = path.extension();
assert_eq!(extension, Some(OsStr::new("txt")));
```

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


### Les extensions de syntaxe


La bibliothèque standard inclut plusieurs extensions de syntaxe. println! est un équivalent au printf de C :
```markdown
let rep = 42
println!("la reponse est {}.", rep);
println!("la reponse est {v}.", rep);
```

Il existe par exemple l’extension asm!, qui permet au développeur d’intégrer du code assembleur en ligne, comme le fait le C via le mot-clé dédié __asm__.

```markdown
#[cfg(target_os = "linux")]
fn helloworld() {
                unsafe {
                asm!(".pushsection .rodata
                msg: .asciz \"Hello World!\"
                .popsection
                lea msg(%rip), %rdi
                call puts");
                        }
                 }
fn main() {
          helloworld();
          }
          
```
Les extensions **error!**, **warn!**, **info!** et **debug!** permettent d’ajouter des traces de log, activables et désactivables via une variable d’environnement.


### Utilisation de Rust sans la bibliothèque standard

La bibliothèque standard de Rust fournit de nombreuses fonctionnalités utiles, mais suppose la prise en charge de diverses fonctionnalités de son système hôte: threads, mise en réseau, allocation de segment de mémoire, etc. Cependant, certains systèmes ne possèdent pas ces fonctionnalités et Rust peut également les utiliser! Pour ce faire, nous disons que la rouille nous ne voulons pas utiliser la bibliothèque standard via un attribut: #![no_std].
Pour l'utiliser #![no_std], ajoutez-le à votre racine de caisse:
```markdown
# ! [ no_std ]

fn  plus_one ( x : i32 ) ->  i32 {
                                 x  +  1 
                                 } 
```
