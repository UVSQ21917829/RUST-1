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

 ### Exercice 2:
  
   écrire un programme rust qui calcul 10 puissance 5 en utilisant la boucle while et indique à l'écran le résultat
   
   
  ### Correction :
   
   
  ```markdown
  
   fn main() {
             let mut x :i32=10;
             let y :i32 =5;
             let mut i :i32 =1;
                while i<y{
                         x=x*10; 
                         i=i+1; 
                         }
              println!("{}",x);
 
               }

 ```
 
## Exercice 3 :
 
 
  écrire un programme rust en utilisant **match** qui indique si un nombre donné est paire ou impaire 
 
  ## Correction :
  
  ```markdown
  
 fn main() {
           let x : i32 = 17;
           let y=x%2;
           match y {
                   0 => {
                        println!("paire !");
                        }
                   _ => {
                         println!("impaire !");
                    }
                   }
 
           }
  
  ```
  
