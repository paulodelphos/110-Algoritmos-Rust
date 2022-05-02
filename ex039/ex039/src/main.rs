//  Faça um programa que leia o ano de nascimento de um jovem e informe
// de acordo com sua idade, se ele ainda vai se alistar ao serviço militar.
// se é a hora de se alistar, ou seja, se ja possou o tempo de alistamento.
// Seu programa também deverá mostrar o tempo que falta ou que passou do prazo.
use std::io;
use chrono::prelude::*;

fn main() {
    let data:DateTime<Utc> = Utc::now();

    let mut nascimento:String = String::from("");

    println!("Digite o ano de nascimento:");
    io::stdin()
        .read_line(&mut nascimento)
        .expect("erro ao ler nascimento");
    let nascimento:i32 = nascimento.trim().parse().expect("Err ao converter nascimento");
    
    
    let idade = data.year() - nascimento;

    println!("Sua idade é {} anos", idade);

    if idade == 18 { 
        println!("Voce pode se alistar agora")
    } else if idade < 18 { 
        println!("VocÊ é menor de 18 anos, nao pode se alistar ainda...");
    } else {
        let atraso = idade - 18;
        println!("Você deveria ter se alistado a {} anos", atraso);
    }


}
