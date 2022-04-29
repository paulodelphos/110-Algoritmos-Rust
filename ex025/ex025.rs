// Crie um programa que leia o nome de uma pessoa e diga se 
// ela tem "Silva" no nome;
use std::io;

fn main() {
    let mut nome:String = String::from("");

    println!("Digite o nome completo: ");
    io::stdin()
        .read_line(&mut nome)
        .expect("Erro ao ler Nome");
    nome.to_uppercase();

    let tem_nome = nome.contains("Silva");

    println!("Tem Silva no nome? {}", tem_nome);


}