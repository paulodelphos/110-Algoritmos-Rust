/*  Crie um programa que leia duas notas de um aluno e calcule sua média,
    mostrando uma mensagem no final, de cacordo com a média atingida:
    - Média abaixo de 5.0: REPROVADO
    - Média entre 5.0 e 6.9: RECUPERAÇÃO
    - Média 7.0 ou SUPERIOR: APROVADO
*/
use std::io;

fn main() {
    let mut nota1:String = String::from("");
    let mut nota2:String = String::from("");

    println!("Digite a primeira nota:");
    io::stdin()
        .read_line(&mut nota1)
        .expect("Erro ao ler nota 1");
    let nota1:f32 = nota1.trim().parse().expect("erro conversao nota 1");

    println!("Digite a segunda nota: ");
    io::stdin()
        .read_line(&mut nota2)
        .expect("Erro leitura nota 2");
    let nota2:f32 = nota2.trim().parse().expect("Erro conversao nota2");

    let media = (nota1 + nota2) / 2.0;
    println!("A média do aluno foi {:.2}", media);
   // let test = 3;
    match media {
        0.0..=4.0 => println!("Aluno Reprovado!"),

        5.0..=6.9 => println!("Aluno em Recuperação"),
        
        7.0..=10.0 => println!("Aluno aprovado!"),

        _ => println!("Valor inválido"),

    }

}
