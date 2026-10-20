
use std::io;

fn main() {
    println!("Devinez le nombre!");
    println!("Entrez le nombre");

    let mut supposition= String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("erreur");

    println!("votre nombre : {}",supposition);
}
