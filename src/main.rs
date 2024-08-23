use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //crea una variable mut de tipo String vacia utilizando la macro new()
    let mut guess = String::new();//la variable toma lo que el usuario ingresa y lo almacena
                                  //aqui no importa cuantas veces almacene un valor (en este caso un String)
                                  //una vez que la 
                                  //variable sale del main (scope) Rust libera la memoria.
                                //mas adelante transformamos el string a número. 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}