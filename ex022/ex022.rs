use std::io;

fn main() {
    let mut nome:String = String::from("");

    println!("Digite o nome: ");
    io::stdin()
        .read_line(&mut nome)
        .expect("Erro ler nome");
    
    let maior = nome.to_uppercase();
    let menor = nome.to_lowercase();
    let listanome: Vec<_> = nome.split(' ').collect();


    println!("Analisando o nome...");
    println!("O seu nome é {}", nome);
    println!("Seu nome em maiusculo é {}", maior);
    println!("Seu nome em minusculo é {}", menor);
    println!("A lista do seu nome é {:?}", listanome[0]);

}