# RUST

  Prise de notes 



#Compilation
on se positionne sur C:\Users\ordinateur\Desktop\mes\mohand
cargo run


#plan fonctionnel
Cette application propose une interface en ligne de commandes pour la l’edition et la gestion des notes. 

#La liste des commandes disponibles dans l'application est la suivante :
 1 Edit :  
Création d’une note.adoc qui n’existe pas 
Modification d’une note.adoc déjà existante.
Delete: Suppression d’un fichier existant 
List : 
  Affichage des notes existantes.
Search: Recherche par :
 titre 
contexte
mot clé
 
View: Affichage d’une note dans le navigateur.



#Documentation
https://doc.rust-lang.org/std/index.html

#Conception de l’application
Pour réaliser cette application  :
   -On a représenté chaque commande(1 edit, list, search, view, exit) sous forme d’une classe et chacune de ces dernières implémente l’interface command , l’interface command permet de commander le récepteur. 
   -l’ensembles des actions (command) sont définis dans la classe Récepteur .
   -La classe invoqueur contient une agrégation de l’interface command et  la méthode inv qui permet de déclencher les differente actions. 

#Le projet peut être amélioré  avec les  fonctionnalités optionnelles.:
la représentation d’une liste  sous forme d’un document JSON 
Utilisation d’un moteur de template . 






