/*  Desenvolva uma lógica que leia o peso e a altura de uma pessoa
Calcule seu IMC e mostre seu status, de acordo com a tabela abaixo:
    - Abaixo de 18.5: Abaixo do peso        - 30 até 40: Obesidade
    - Entre 18.5 e 25: Peso ideal           - Acima de 40: Obesidade mórbida
    - 25 até 30: Sobrepeso

*/
use std::io;

fn main() {

    let mut peso:String = String::from("");
    let mut altura:String = String::from("");

    println!("Digite o peso: ");
    io::stdin()
        .read_line(&mut peso)
        .expect("Erro na leitura de peso");
    let peso:f32 = peso.trim().parse().expect("Erro na conversao de peso");

    println!("Digite a altura");
    io::stdin()
        .read_line(&mut altura)
        .expect("Erro na leitura de altura");
    let altura:f32 = altura.trim().parse().expect("Erro na conversao de altura");

    let imc = peso / (altura.powf(2.0));
    
    println!("O seu imc é de: {:.2}", imc);
    if  imc < 18.5 {
        println!("Abaixo do peso");
    } else if imc >= 18.8 && imc < 25.0 {
        println!("Peso ideal");
    } else if imc >= 25.0 && imc < 30.0 {
        println!("Sobrepeso");
    } else if imc >= 30.0 && imc < 40.0 {
        println!("Obesidade");
    } else {
        println!("Obesidade Mórbida - cuidado!");
    }

}
