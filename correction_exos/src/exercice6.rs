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
fn main() {
    println!(" pgcd(48,18) : {}",pgcd(48,18));
    println!("pgcd(45,15) : {}",pgcd(45,15));
    println!("pgcd(76,19) : {}",pgcd(76,19));
    println!("pgcd(47,7) : {}",pgcd(47,7));
    println!("pgcd(107,96) : {}",pgcd(107,96));
    
}