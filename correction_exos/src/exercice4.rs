
fn main() {
    //remplir le tableau
    let array: [i32; 5] = [0, 1, 2, 3, 4];

    // affichage
    let mut compteur = 0;
    for x in array.iter(){
    println!("l'element a l'index {} est {}", compteur, x);
    compteur += 1;
}
