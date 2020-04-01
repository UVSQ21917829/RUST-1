[**Base du langage**](https://UVSQ21917829.github.io/RUST-1)-----[**Concepts avancés**](https://UVSQ21917829.github.io/RUST-1/concept)-----[**Bibliothèques standards**](https://UVSQ21917829.github.io/RUST-1/std)-----[**Outils de développement**](https://UVSQ21917829.github.io/RUST-1/index2)-----[**Ressources d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/rsc)-----[**Exercices d'apprentissages**](https://UVSQ21917829.github.io/RUST-1/exo)


## Les Bibliothèques tierce en RUST

### Crates.io

**Crates** est un peu similaires aux paquets de certaines autres langues. Les caisses sont compilées individuellement. Si la caisse a des modules de fichiers enfants, ces fichiers seront fusionnés avec le fichier caisse et compilés en une seule unité.

Une caisse peut produire un exécutable / un binaire ou une bibliothèque. src/main.rsest la racine de la caisse / le point d'entrée d'une caisse binaire et src/lib.rsle point d'entrée d'une caisse de bibliothèque.


Ici nous utilisons  des exemples les plus simples de quelques bibliothèque tierces 

## libc - Liaisons brutes FFI avec les bibliothèques système des plateformes

libcfournit toutes les définitions nécessaires pour interagir facilement avec le code C (ou le code "C-like") sur chacune des plates-formes prises en charge par Rust. Cela inclut les définitions de type (par exemple c_int), les constantes (par exemple EINVAL) ainsi que les en-têtes de fonction (par exemple malloc).

Cette caisse exporte tous les types de plates-formes, fonctions et constantes sous-jacents sous la racine de la caisse, de sorte que tous les éléments sont accessibles en tant que libc::foo. Les types et les valeurs de toutes les API exportées correspondent à la plate-forme pour laquelle libc est compilée.


## Rand

Une bibliothèque Rust pour la génération de nombres aléatoires.

Rand fournit des utilitaires permettant de générer des nombres aléatoires, de les convertir en types et distributions utiles, ainsi que certains algorithmes liés à l’aléatoire.

Les principales caractéristiques de génération de nombres aléatoires de Rand résident dans la caisse rand_core mais sont également exposées ici; Les implémentations de RNG devraient préférer être utilisées, rand_corealors que la plupart des autres utilisateurs devraient en dépendre rand.

## log
Une bibliothèque Rust offrant une façade de journalisation légère .


Une façade de journalisation fournit une API de journalisation unique qui résume l'implémentation de la journalisation réelle. Les bibliothèques peuvent utiliser l'API de journalisation fournie par cette caisse et le consommateur de ces bibliothèques peut choisir l'implémentation de journalisation la mieux adaptée à son cas d'utilisation.

## regex

Une bibliothèque Rust pour analyser, compiler et exécuter des expressions régulières. Sa syntaxe est similaire à celle des expressions régulières de style Perl, mais il lui manque quelques fonctionnalités telles que le regard autour et les références arrières. En échange, toutes les recherches s'exécutent en temps linéaire par rapport à la taille de l'expression régulière et du texte de recherche. Une grande partie de la syntaxe et de la mise en œuvre est inspirée de RE2 .

## Syn 

Syn est une bibliothèque d’analyses permettant d’analyser un flux de jetons Rust dans une arborescence de syntaxe de code source Rust.

Actuellement, cette bibliothèque est destinée à être utilisée dans les macros procédurales de Rust, mais contient des API pouvant être utiles de manière plus généraleLes principaux objectifs de cette bibliothèque sont:

Portabilité: devrait fonctionner sur n'importe quel système (Unix ou Windows).
Support: a été écrit pour un projet réel (Pijul), il est donc peu probable que le support prenne fin.
Qualité de sortie: évitez les lignes de terminal clignotantes habituelles des anciennes bibliothèques C.

## jauge

Une bibliothèque de mesures à guichet unique pour les applications Rust avec de nombreuses fonctionnalités, 
un impact minimal sur les applications et un choix de sorties vers les systèmes en aval.

## lazy-static.rs

Une macro pour déclarer une statique évaluée paresseusement dans Rust.

À l’aide de cette macro, il est possible d’avoir staticbesoin d’exécuter du code lors de l’exécution pour pouvoir être initialisé. Cela inclut tout ce qui nécessite des allocations de tas, comme des vecteurs ou des cartes de hachage, ainsi que tout ce qui nécessite le calcul d'appels de fonctions non const.

## syscallz-rs

Bibliothèque simple seccomp pour la rouille. Veuillez noter que la liste des appels système est incomplète et que vous devrez peut-être envoyer un PR pour que vos appels système soient inclus. Cette caisse se libère fréquemment si la liste d'appels système a été mise à jour.

## wincolor

Une API simple spécifique à Windows pour contrôler la couleur du texte dans une console Windows. Le but de cette caisse est d'exposer la rigidité totale de la console Windows sans aucune abstraction indépendante de la plate-forme.

## difference.rs    
Bibliothèque de différences de texte Rust avec assertion de différences intégrée.
 ### exemple 
 
 ```markdown
 use difference::Changeset;

let changeset = Changeset::new("test", "tent", "");

assert_eq!(changeset.diffs, vec![
              Difference::Same("te".to_string()),
              Difference::Rem("s".to_string()),
              Difference::Add("n".to_string()),
              Difference::Same("t".to_string())
                                 ]
           );
```

## bitflags


Une macro Rust pour générer des structures qui se comportent comme un ensemble de drapeaux bits

## solicit

Une implémentation HTTP / 2 dans Rust.

L'objectif principal du projet est de fournir une implémentation de bas niveau du protocole HTTP / 2 et de l'exposer de manière à ce que les bibliothèques de niveau supérieur puissent l'utiliser. Par exemple, il devrait être possible pour une bibliothèque de niveau supérieur d'écrire un simple adaptateur qui expose les réponses obtenues via une connexion HTTP / 2 de la même manière que celles obtenues via HTTP / 1.1

## libffi 

La bibliothèque C libffi offre deux fonctionnalités principales: assembler des appels à des fonctions de manière dynamique et créer des fermetures pouvant être appelées en tant que fonctions C ordinaires. Dans Rust, ce dernier signifie que nous pouvons transformer un Rust lambda (ou tout objet implémentant Fn/ FnMut) en un pointeur de fonction C ordinaire que nous pouvons transmettre en tant que rappel à C.
## ocaml

ocaml-rspermet d’écrire des extensions OCaml directement dans Rust, sans talon C. Il a été créé par raml dans le but de créer une interface sécurisée de haut niveau.

## android_base

android_base des applications Android fournit une api pour développer des applications en RUST pour Android sans se soucier des détails spécifiques à Android, tels que les implémentations opengl ou la boucle d’événement, tout en ayant ses fonctions exposées très abstraites par rapport à ce que vous pouvez faire

## file
Cette caisse est obsolète depuis Rust 1.26. Si vous avez un projet Rust lié à un fichier et souhaitez utiliser ce nom de caisse, 
**String**
file::get_text()et file::put_text()- lire et écrire Stringavec un seul appel de fonction.
Utilisez std::fs::read_to_string("path")?et et std::fs::write("path", string)?dans Rust 1.26 ou une version ultérieure.

## google-geo
BIBLIOTHÈQUE DE RECHERCHE D'INFORMATIONS GEO
Utilisation de la longitude et de la latitude pour rechercher des informations géographiques à l'aide du service de carte Google.

## google-mapsengine1
Une bibliothèque complète pour interagir avec Maps Engine (protocole v1)

## image
Une bibliothèque de traitement d'images
Cette caisse fournit des fonctions de base de traitement d’imagerie et des méthodes de conversion vers et à partir de formats d’image.

Toutes les fonctions de traitement d'image fournies fonctionnent sur des types qui implémentent le GenericImagetrait et renvoient un ImageBuffer.

## mail-smtp  
Permet l'envoi mail-core Mailvia new-tokio-smtp

Cette bibliothèque lie new-tokio-smtpet les mail caisses.

## macros-html

Cette caisse fournit la html!macro pour la construction de documents HTML dans votre code Rust en utilisant une syntaxe approximativement compatible JSX .

## pdf
Lire, modifier et écrire des fichiers PDF.

Pour le moment, vous ne pouvez lire que les fichiers PDF.

Une façon simple de contribuer est d'ajouter différents fichiers PDF tests/fileset de voir s'ils réussissent les tests ( cargo test).

## stdweb

L'objectif de cette caisse est de fournir des liaisons Rust aux API Web et de permettre un haut degré d'interopérabilité entre Rust et JavaScript.

## find-files
L'utilitaire de recherche de fichiers (ff) recherche de manière récursive les fichiers dont les noms correspondent au modèle RegExp spécifié dans le répertoire fourni (par défaut, le répertoire actuel, s'il n'a pas été fourni)

## git-project
git-project vous permet de gérer rapidement et facilement le grand nombre de projets git pouvant s'accumuler sur l'ordinateur d'un développeur moderne.

### Utiliser crates.io
Pour télécharger les bibliothèques sur [**crates.io**](https://crates.io/),

Nous devons créer un compte sur crates.io pour acquérir un jeton API
Puis exécutez-vous cargo login <token>avec ce jeton API .
