fn main() {
    let x : i32 = 22;
    let y=x%2;
    match y {
            0 => {
                 println!("{} est paire !",x);
                 }
            _ => {
                  println!("{} est impaire !",x);
             }
            }
}

    