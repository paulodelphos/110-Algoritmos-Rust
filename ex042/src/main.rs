// Mostre qual triângulo será formado caso seja possivel
// - Equilátero: todos os lados iguais
// - Isósceles: todos os lados iguais
// Escaleno: Todos os lados diferentes
use std::io;

fn main() {
    let mut r1:String = String::from("");
    let mut r2:String = String::from("");
    let mut r3:String = String::from("");

    println!("Digite o primeiro lado:");
    io::stdin()
        .read_line(&mut r1)
        .expect("Err read to r1");
    let r1:i32 = r1.trim().parse().expect("Erro conversao r1");

    println!("Digite o segundo valor: ");
    io::stdin()
        .read_line(&mut r2)
        .expect("Erro leitura de r2");
    let r2:i32 = r2.trim().parse().expect("Erro conversao r2");

    println!("Digite o terceiro valor");
    io::stdin()
        .read_line(&mut r3)
        .expect("Erro leituar de r3");
    let r3:i32 = r3.trim().parse().expect("Erro conversao r3");

    if r1 < r2 + r3 && r2 < r1 + r3 && r3 < r1 + r2 {
        println!("Os lados formam um triângulo.");

        if r1 == r2 && r1 == r3 {
            println!("É um triangulo EQUILÁTERO.");
        } else if r1 == r2 || r1 == r3 || r2 == r3 {
            println!("É um triângulo ISÓCELES")
         } else {
             println!("É um triângulo ESCALENO")
         }

    } else {
        println!("Os lados não podem formar um triângulo.")
    }


}
