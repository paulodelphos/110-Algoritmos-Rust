// Crie um programa que leia um número inteiro e mostre na tela se é PAR ou IMPAR.
use std::io;
fn main() {

    let mut num:String = String::from("");

    println!("Digite um número inteiro qualquer:");
    io::stdin()
        .read_line(&mut num)
        .expect("Erro ao ler num");
    let num:u32 = num.trim().parse().expect("Erro ao converter num");

    let par = if num % 2 == 0 {"É par!"} else {"É impar!"};
        
    println!("{}", par);
}
