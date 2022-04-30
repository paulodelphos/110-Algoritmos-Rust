// Desenvolva um programa que leia o comprimento de três retas e diga ao usuário se elas podem ou n não formar um triângulo
use std::io;

fn main() {
    let mut lado1:String = String::from("");
    let mut lado2:String = String::from("");
    let mut lado3:String = String::from("");

    println!("Digite o primeiro lado: ");
    io::stdin()
        .read_line(&mut lado1)
        .expect("Erro ao ler lado 1");
    let lado1:f32 = lado1.trim().parse().expect("erro o converter lado1");

    println!("Digite o segundo lado: ");
    io::stdin()
        .read_line(&mut lado2)
        .expect("Erro ao ler lado 2");
    let lado2:f32 = lado2.trim().parse().expect("Erro ao converter lado2");

    println!("Digite o terceiro lado: ");
    io::stdin()
        .read_line(&mut lado3)
        .expect("Erro ao ler lado 3");
    let lado3:f32 = lado3.trim().parse().expect("Erro ao converter lado3");

    if lado1 <= lado2 + lado3 && lado2 <= lado1 + lado2 && lado3 <= lado1+ lado2 {
        println!("Os lados {:.2}, {:.2}, {:.2}, formam um triângulo!", lado1, lado2, lado3);
    } else {
        println!("Os lados {:.2}, {:.2}, {:.2}, Não formam um triângulo", lado1, lado2, lado3);
    }
}