/*  Esqueva um programa para aprovar o emprestimo bancário para a compra de uma casa.
    Pergunte o valor da casa, o salário do comprador e quantos anos para pagar.
    A prestação mensao nao pode exceder 30% do salário ou o emprestimo será negado.
*/
use std::io;

fn main() {

    let mut valor_casa:String = String::from("");
    let mut salario:String = String::from("");
    let mut anos:String = String::from("");

    println!("Informe o valor da casa: ");
    io::stdin()
        .read_line(&mut valor_casa)
        .expect("Erro leitura valor da casa");
    let valor_casa:f64 = valor_casa.trim().parse().expect("Error conversao valor da casa");

    println!("Informe quantos o tempo da prestação em anos");
    io::stdin()
        .read_line(&mut anos)
        .expect("Erro ler anos");
    let anos:f64 = anos.trim().parse().expect("Error conversao anos");

    println!("Informe o valor do salário: ");
    io::stdin()
        .read_line(&mut salario)
        .expect("error ao converter salario");
    let salario:f64 = salario.trim().parse().expect("Error conversao salario");

    let meses = anos * 12.00; 
    let mensalidade = valor_casa / meses;

    let analise_credito = if  mensalidade <= salario * 0.3{"Empréstimo Aprovado!"} else {"Emprestimo Reprovado."};

    println!(
        "A mensalidade de R$ {:.2} em {} meses ficou em R$ {:.2} reais. \nA análise do crédito foi: {}",
        valor_casa, meses, mensalidade,  analise_credito
    );
}