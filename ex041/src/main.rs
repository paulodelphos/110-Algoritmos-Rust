/*  A confederação Nacional de natação precisa de um programa que leia o ano
de nascimento de um atleta e mostre sua categoria, de acordo com a idade:
    - Até 9 anos: MIRIM     - Até 19 anos: Junior
    - Até 14 anos: INFANTIL - Até 25 anos: Sênior
    - Acima: MASTER
*/
use std::io;
use chrono::prelude::*;

fn main() {
    let data:DateTime<Utc> = Utc::now();

    let mut nascimento:String = String::from("");
    
    println!("Digite o ano de nascimento");
    io::stdin()
        .read_line(&mut nascimento)
        .expect("Erro ao ler nascimento");
    let nascimento:i32 = nascimento.trim().parse().expect("Err conversao de nascimento");
    let idade = data.year() - nascimento;

    println!("A idade é de {} anos", idade);

    match idade {
        0..=9 => { println!("Categorina: MIRIM.") },
        10..=14 => { println!("Categoria: INFANTIL") },
        15..=19 => { println!("Categoria: JUNIOR ")},
        20..=25 => { println!("Categoria: SÊNIOR")},
        _ => { println!("Categoria: MASTER")}
    }


}
