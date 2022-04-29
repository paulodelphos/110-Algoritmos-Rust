use std::io;
use rand::Rng;

fn main() {
   let mut n1:String = String::from("");
   let mut n2:String = String::from("");
   let mut n3:String = String::from("");
   let mut n4:String = String::from("");

    println!("Nome do primeiro aluno:");
    io::stdin()
        .read_line(&mut n1)
        .expect("Erro leitura n1");
    
    println!("Nome do segundo aluno: ");
    io::stdin()
        .read_line(&mut n2)
        .expect("Erro leitura n2");
    
    println!("Nome do terceiro aluno: ");
    io::stdin()
        .read_line(&mut n3)
        .expect("Erro ler n3");

    println!("Nome do quarto aluno: ");
    io::stdin()
        .read_line(&mut n4)
        .expect("Erro ler n4");

    //let mut lista:Vec<String> = Vec::new();
   let lista = vec![n1, n2, n3, n4];
    //lista.push(n1);
    //lista.push(n2);

    let escolha = rand::thread_rng().gen_range(0..4);

    println!("O aluno escolhido foi: {}", lista[escolha]);


}