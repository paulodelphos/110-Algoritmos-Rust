// Fala um programa que leia três números e mostre qual é o maior e menor deles
use std::io;

fn main() {
    let mut n1: String = String::from("");
    let mut n2: String = String::from("");
    let mut n3: String = String::from("");

    println!("Digite o primeiro valor: ");
    io::stdin().read_line(&mut n1).expect("Err ao ler n1");
    let n1: i32 = n1.trim().parse().expect("Erro ao converter n1");

    println!("Digite o segundo valor: ");
    io::stdin().read_line(&mut n2).expect("Erro ao ler n2");
    let n2: i32 = n2.trim().parse().expect("Err ao converter n2");

    println!("Digite o terceiro valor: ");
    io::stdin().read_line(&mut n3).expect("error ao ler n3");
    let n3: i32 = n3.trim().parse().expect("Erro ao converter n3");

    let mut menor = n1;
    let mut maior = n1;

    // Verificar o menor
    if n2 < n1 && n2 < n3 {
        menor = n2;
    };
    if n3 < n1 && n3 < n2 {
        menor = n3;
    };
    // verificar o maior
    if n2 > n1 && n2 > n3 {
        maior = n2;
    }
    if n3 > n2 && n3 > n1 {
        maior = n3;
    }

    println!("O Maior número é {}", maior);
    println!("O Menor número é {}", menor);
}
