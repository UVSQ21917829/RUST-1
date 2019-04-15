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

### Exercice 4 :

  écrire un programme rsut qui affiche les éléments d'un tableau 
  
### Corrction :

```markdown
fn main() {
          let mut t : Vec<i32> = Vec::new();
          t.push(0);
          t.push(1);
          t.push(2);
          let s = &t;
          println!("{:?}",s);
          }

``` 

### Exercice 5 :

  écrire un programme rust qui calcul les éléments d'un tableau  

### Corrction :

```markdown

fn main() {
          let  v = [0 ,1 ,2];
          let mut i=0;
          let mut somme =0;
          let s = &v;
          while i<3{
                   somme=somme+s[i];
                   i=i+1;
                   }
                   println!("{:?}",somme);
          }

```

### Exercice 6 :

Ecrire un programme avec la fonction pgcd qui calcul le pgcd
 des deux fonctions. La méthode main doit tester cette
fonction sur plusieurs exemples. 

### Corrction : 

```markdown

fn pgcd(mut x :i32,mut y:i32)->i32{
         let mut i :i32=0;
         while i==0{
                    if x>y{
                          x=x-y;  
                          if x==0{
                          i=y;
                          }
                          }else {
                                y=y-x; 
                                if y==0{
                                       i=x;
                                       }
                                }
                          }
           return i;
                                    }
```

```markdown
  fn main() {
            println!("{}",pgcd(48,18));
            println!("{}",pgcd(45,15));
            println!("{}",pgcd(76,19));
            println!("{}",pgcd(47,7));
            println!("{}",pgcd(107,96));
            
            }

```
### Exercice 7 :

 Ecrire une fonction qui cacul la devision entre deux fonction , cette fonction doit être associée à des tests unitaires

### Corrction : 

```markdown
pub fn div(a: u32, b: u32) -> u32 {
          if b == 0 {
                    panic!("Division sur zero !");
                    } 
          a / b
                                  }

#[cfg(test)]
mod tests {
         use super::*;
         #[test]
         fn test_div() {
                       assert_eq!(div(10, 2),5);
                       }

         #[test]
         #[should_panic]
         fn test_div_sur_zero() {
                                div(1, 0);
                                }

          }

```
