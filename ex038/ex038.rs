// Escreva um programa que leia dois números inteiros e compare-os mostrando na tela uma msg:
//  - O primeiro valor é maior
//  - O segundo valor é maior
// - Não existe valor maior, os dois são iguais
use std::io;

fn main() {
    let mut primeiro:String = String::from("");
    let mut segundo:String = String::from("");

    println!("Digite o primeiro valor: ");
    io::stdin()
        .read_line(&mut primeiro)
        .expect("Erro leitura do primeiro valor");
    let primeiro:i32 = primeiro.trim().parse().expect("Erro conversao de primeiro");

    println!("Digite o segundo valor: ");
    io::stdin()
        .read_line(&mut segundo)
        .expect("Erro leitura do segundo valor");
    let segundo:i32 = segundo.trim().parse().expect("Erro conversao do segundo valor");

    if primeiro > segundo {
        println!("Primeiro valor é maior");
    } else if segundo > primeiro {
        println!("Segundo valor é maior");
    } else {
        println!("Os dois valores são iguais!");
    }
}