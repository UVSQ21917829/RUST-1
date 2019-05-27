## Correction des exercices :

### Exercice 1 :

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
 
### Exercice 3 :

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

### Exercice 8:

```markdown
use std::fs::File;
use std::io::prelude::*;

fn main()  {
    let mut file = File::create("rust.txt")?;
    file.write_all(b"bonjour, le monde")?;
   
}

```
