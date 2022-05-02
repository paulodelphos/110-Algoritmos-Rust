// Escreva um programa que leia um número inteiro qualquer e peça
// para o usuário escolher qual será a base de conversão:
//   - 1 para binário
//   - 2 para octonl
//   - 3 para hexadecimal
use std::io;

fn main() {

    let mut num:String = String::from("");
    let mut opcao:String = String::from("");

    println!("Digite um número interio: ");
    io::stdin()
        .read_line(&mut num)
        .expect("err leitura num");
    let num:i32 = num.trim().parse().expect("erro conversao num");

    println!(
        "=-===-=-=-=-=-=-=-=-=-=-=-=-=-=-=
        \nDigite uma opção de conversão:
        \n 1 para binário
        \n 2 para octal
        \n 3 para hexadecimal
        \n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
        "
    );
    io::stdin()
        .read_line(&mut opcao)
        .expect("Erro leitura da opção");
    let opcao:u8 = opcao.trim().parse().expect("Erro conversao da opção");
    
    match opcao {
        1 => {
             println!("O valor em binário é {:b}\n\n", num);
            }
        2 => {
            println!("O valor em Octal é {:o}\n\n", num);
        }
        3 => {
            println!("O valor e Hexadecimal é {:e}\n\n", num);
        }
        _ => { println!("Opção inválida!\n\n"); }
    }
   
}