//Faça um programa que leia um número de 0 a 999 e mostre
// na tela cada um dos digidtoi separados
use std::io;

fn main() {
    let mut num:String = String::from("");

    println!("Digite um numero até 999");
    io::stdin()
        .read_line(&mut num)
        .expect("Erro na leitura de num");
    let num:f32 = num.trim().parse().expect("Erro na conversao");

    let unidade  = num / 1.0 % 10.0;
    let dezena = num / 10.0 % 10.0;
    let centena = num / 100.0 % 10.0;

    println!("Voce digitou {}", num);
    println!("A unidade é {}", unidade);
    println!("A dezena é {:.1}", dezena);
    println!("A centena é {:.1}", centena);
}