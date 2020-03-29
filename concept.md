# Les concepts avancés
## Les pointeurs intelligents (Smart pointer) 

Dans Rust, les pointeurs intelligents ne sont pas seulement des pointeurs mais également une structure de données. Ce concept de pointeurs intelligents n'est pas seulement dans le language Rust, il existe aussi dans d'autres lagnguages et ils sont sont originaires de C ++. Dans Rust, les pointeurs intelligents définis dans la bibliothèque standard offrent des fonctionnalités au-delà de celles fournies par les références.
### Box
Box est le pointeur intelligent le plus simple qui permet de stocker des données sur le tas (Heap). Nous utilisons le type de Box pour fournir la taille du type afin que le compilateur sache combien de mémoire doit être allouée.
