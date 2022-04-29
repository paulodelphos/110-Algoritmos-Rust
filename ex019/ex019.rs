//use std::io;
use rand::Rng;

fn main() {
   let  n1:String = String::from("a");
   let  n2:String = String::from("b");
   let  n3:String = String::from("c");
   let  n4:String = String::from("d");

    //let mut lista:Vec<String> = Vec::new();
    let lista = vec![n1, n2, n3, n4];
    //lista.push(n1);
    //lista.push(n2);

    let escolha = rand::thread_rng().gen_range(0..4);

    println!("Valor do vetor é {:?}", lista[1]);
    println!("Valor do rand é {}", escolha);


}