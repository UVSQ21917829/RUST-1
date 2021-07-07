// exercice dans lib.rs
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
                 assert_eq!(div(10, 2),4);
   }

   #[test]
   #[should_panic]
   fn test_div_sur_zero() {
                          div(1, 0);
}
}