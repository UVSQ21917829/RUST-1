## Exercices D'apprenstissage :

### Exercice 1 :
   écrire en deux méthode le programme qui calcule la factorielle de 10 et indique à l'écran le résultat.
     


### Correction :
   
   ### méthode 1:
    
 ```markdown
    
   fn main() {
   let mut n :i32 =10;
   let mut i :i32 =1;
   loop {
      n=n*i;
      i=i+1;
      if i==10{
          break;
       }
    }
    println!("{}",n);
    
}
```

### méthode 2:
    
 ```markdown
   fn main() {
    let mut n :i32 =10;
    let mut i :i32=1;
    
    while i<10{
        n=n*i;
        i=i+1;
    }
    println!("{}",n);
}
```
     
