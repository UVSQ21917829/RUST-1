[**Base du langage**](https://uvsq21807686.github.io/RUST)-----[**Bibliothèques standards**](https://uvsq21807686.github.io/RUST/std)-----[**Outils de développement**](https://uvsq21807686.github.io/RUST/index2)-----[**Ressources d'apprentissages**](https://uvsq21807686.github.io/RUST/rsc)  


## Les Bibliothèques tierce en RUST

### Crates.io

**Crates** est un peu similaires aux paquets de certaines autres langues. Les caisses sont compilées individuellement. Si la caisse a des modules de fichiers enfants, ces fichiers seront fusionnés avec le fichier caisse et compilés en une seule unité.

Une caisse peut produire un exécutable / un binaire ou une bibliothèque. src/main.rsest la racine de la caisse / le point d'entrée d'une caisse binaire et src/lib.rsle point d'entrée d'une caisse de bibliothèque.


Ici nous utilisons  des exemples les plus simples de quelques bibliothèque tierces 

## libc - Liaisons brutes FFI avec les bibliothèques système des plateformes

libcfournit toutes les définitions nécessaires pour interagir facilement avec le code C (ou le code "C-like") sur chacune des plates-formes prises en charge par Rust. Cela inclut les définitions de type (par exemple c_int), les constantes (par exemple EINVAL) ainsi que les en-têtes de fonction (par exemple malloc).

Cette caisse exporte tous les types de plates-formes, fonctions et constantes sous-jacents sous la racine de la caisse, de sorte que tous les éléments sont accessibles en tant que libc::foo. Les types et les valeurs de toutes les API exportées correspondent à la plate-forme pour laquelle libc est compilée.


## Rand
Statut de construction Statut de construction Caisse Livre API API Version minimum rustc

Une bibliothèque Rust pour la génération de nombres aléatoires.

Rand fournit des utilitaires permettant de générer des nombres aléatoires, de les convertir en types et distributions utiles, ainsi que certains algorithmes liés à l’aléatoire.

Les principales caractéristiques de génération de nombres aléatoires de Rand résident dans la caisse rand_core mais sont également exposées ici; Les implémentations de RNG devraient préférer être utilisées, rand_corealors que la plupart des autres utilisateurs devraient en dépendre rand.

## log
Une bibliothèque Rust offrant une façade de journalisation légère .

Statut de construction Statut de construction Dernière version Documentation Licence
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
