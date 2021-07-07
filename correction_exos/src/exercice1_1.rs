
fn main() {
    let n :i32 =10;
    let mut i :i32 =1;
    let mut result :i32 =n;
    loop {
        result=result*i;
         i=i+1;
         if i==10{
                 break;
                 }
    }
    println!("le factoriel de {} est {}",n,result);

}