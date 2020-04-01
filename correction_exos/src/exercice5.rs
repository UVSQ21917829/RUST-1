fn main() {

    let  v = [0 ,1 ,2];
    let mut i=0;
    let mut somme =0;
    let s = &v;
    while i<3{
             somme=somme+s[i];
             i=i+1;
    }
    println!("la somme est {:?}",somme);
}