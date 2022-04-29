// Faça um programa que leia o nome completo de uma pessoa,
// mostrtando em seguida o primeiro e último nome separadamente
// Ex; Ana Maria de Souza
//Primeiro: Ana
// último: Souza

fn main() {
    let nome:String = String::from("Ana Maria de Souza");

    let vetor_nome: Vec<&str> = nome.split(' ').collect();

    let primeiro = vetor_nome.first();
    let ultimo = vetor_nome.last();

    println!("{:?}", vetor_nome);
    println!("Primeiro nome: {:?}", primeiro);
    println!("Último nome: {:?}", ultimo);
}

