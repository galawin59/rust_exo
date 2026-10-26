use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre!");

    let nombre_secret = rand::thread_rng().gen_range(1..101);
    println!("Le nombre secret est : {}", nombre_secret);
    loop{
    println!("Entrez le nombre");

    let mut supposition= String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("erreur");

    let supposition: u32 = match supposition.trim().parse() {
        Ok(nombre) => nombre,
        Err(_) => continue,
    };
    println!("votre nombre : {}",supposition);

    match supposition.cmp(&nombre_secret){
        Ordering::Less => println!("C'est plus!"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => {
            println!("Bingo!!");
            break;  
        }              
    }
}
}
