/*  Elabore um programa que calcule o valor a ser pago por um produto,
considerando o seu preço normal e condição de pgamento:
- a vista dinheiro/cheque 10% de desconto              - em até 2x no cartão: preço normal
- à vista no cartão: 5% de desconto                     2x ou mais no cartao: 20% de juros

*/
use std::io;

fn main() {
    let mut valor:String = String::from("");
    let mut pagamento:String = String::from("");
    
    println!("Qual o preço das compras?");
    io::stdin()
        .read_line(&mut valor)
        .expect("Erro ao ler o valor");
    let valor:f32 = valor.trim().parse().expect("Error converter o valor");

    println!("========== LOJAS DELPHOS =========");
    println!("Preço das compras: {}", valor);
    println!("FORMAS DE PAGAMENTO:");
    println!(
        "[1] à vista dinheiro/cheque
        \n[2] à vista no cartão
        \n[3] 2x no cartão
        \n[4] 3x ou mais vezes no cartão
        "
    );
    println!("Escolha a opção: ");
    io::stdin()
        .read_line(&mut pagamento)
        .expect("Erro ao ler pagamento");
    let pagamento:u16 = pagamento.trim().parse().expect("erro na conversao de pagamento");

    match pagamento {
        1 => { println!("Valor a vista ficou R${:.2} reais", valor * 0.9); },
        2 => { println!("Valor a vista no cartão R${:.2} reais", valor * 0.95); },
        3 => { println!("Valor em duas vezes no cartao R${:.2} reais", valor); },
        4 => { println!("Valor em 3x ou mais no cartão R$ {:.2} reais", valor * 1.2); },
        _ => { println!("Opção inválida!")}
    }
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
}
