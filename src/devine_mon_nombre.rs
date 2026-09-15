use std::io;

fn main() {
    println!("Devine mon nombre !");
    println!("Saisissez votre proposition.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Votre nombre: {}", input.trim());
}