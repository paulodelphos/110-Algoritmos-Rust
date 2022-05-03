// Criar jogo Pedra-Papel-Tesoura
use std::io;
use rand::Rng;

fn main() {

    let mut opcao:String = String::from("");

    let sorte = rand::thread_rng().gen_range(0..2);

    println!("Sua opções:
    [0] PEDRA    
    [1] PAPEL
    [2] TESOURA
    Qual é a sua jogada?
    ");

    io::stdin()
        .read_line(&mut opcao)
        .expect("Erro ao ler opção.");
    let opcao:u32 = opcao.trim().parse().expect("erro ao converter opcao");
    
    println!("JO QUEM PHO...
        Eu joguei {} e você jogou {}",
        sorte, opcao
    );
    
    if opcao == sorte {
         println!("Empatamos");
    } else if opcao == 0 && sorte == 1 {
        println!("Você perdeu!");
    } else if opcao == 0 && sorte == 2 {
        println!("VocÊ ganhou")
    } else if opcao == 1 && sorte == 0 {
        println!("Voce perdeu!") 
    } else if opcao ==1 && sorte == 2 {
        println!("Voce ganhou!")
    } else if opcao == 2 && sorte == 0 {
        println!("Voce perdeu!")
    } else {
        println!("Voce ganhou!!")
    }

    
}
