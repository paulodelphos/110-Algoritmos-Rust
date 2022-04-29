// Crie um programa que leia o nome de uma cidade e diga
// se ela começa ou não com o nome "SANTO"
use std::io;

fn main() {



    let mut cidade:String = String::from("");
    
    println!("Digite o nome de uma cidade: ");
    io::stdin()
    .read_line(&mut cidade)
    .expect("Erro ao ler cidade");
    
    cidade = cidade.to_lowercase();
    let slice = &cidade[0..5];
    let tem_santo = if slice == "santo" {"Tem santo"} else {"Nao tem santo"};
    println!("{}", tem_santo);

   
    
    println!("{}", slice);
}


