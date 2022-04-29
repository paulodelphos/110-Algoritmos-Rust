use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;



fn main() {
    
    let mut n1:String = String::from("");
    let mut n2:String = String::from("");
    let mut n3:String = String::from("");
    let mut n4:String = String::from("");

    println!("Primeiro nome: ");
    io::stdin()
        .read_line(&mut n1)
        .expect("error read n1");

    println!("Segundo nome: ");
    io::stdin()
        .read_line(&mut n2)
        .expect("error read n2");
    
    println!("Terceiro nome: ");
    io::stdin()
        .read_line(&mut n3)
        .expect("erro read n3");

    println!("Quarto nome: ");
    io::stdin()
        .read_line(&mut n4)
        .expect("error read to n4");



    let mut rng = thread_rng();
    let mut lista = [n1, n2, n3, n4];

    lista.shuffle(&mut rng);

    println!("\n=-==-=-=-=-=-=");
    for i in 0..4 {
        
        println!("{} ", lista[i])
    } 
    println!("=-==-=-=-=-=-=");

}
