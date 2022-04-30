// Escreva um programa que leia a velocidade de um carro.
// se ele ultrapassar 80km/h, mostre uma mensagem dizendo que foi multado.
// A multa vai custar R$ 7.00 por kada km acima do limite
use std::io;

fn main() {

    let mut velocidade:String = String::from("");

    println!("Digite a velocidade atual do carro:");
    io::stdin()
        .read_line(&mut velocidade)
        .expect("Erro ao ler velocidade");
    let velocidade:i32 = velocidade.trim().parse().expect("Erro ao converter velocidade");


    match velocidade {
        0..= 80 => {
            println!("Sua velocidade é {} km/h \nTenha uma boa viagem", velocidade);
        }
        _ => {
            println!("Sua velocidade é {} km/h \nVocê foi multado! Em {} reais.", velocidade, (velocidade - 80) * 7)
        }
    }
  
}
