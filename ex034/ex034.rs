// Escreva um programa que pergunte o salário de um funcionário e calcule o valor
// do seu aumento.
//Para salário superiores a R$ 1.250,00, calcule um aumento de 10%.
// Para igual ou inferior, aumento de 15%
use std::io;

fn main () {
    let mut salario:String = String::from("");

    println!("Digite o valor do Salário: ");
    io::stdin()
        .read_line(&mut salario)
        .expect("erro ao ler salario");
    let salario:f32 = salario.trim().parse().expect("Erro conversao salario");

    let aumento = if salario <= 1250.00 { salario * 1.15} else {salario * 1.10};

    println!("
        O valor do sálário R${} com aumento ficou em R${} reais.\n\n",
        salario, aumento
    )
    
}