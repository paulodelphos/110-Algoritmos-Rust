// Desenvolva um program que pergunt a distância de uma viagem em KM.
// Calcule o preço da passagem, cobrando R$ 0.50 por Km para
// viagens de até 200km e R$0.45 para viagens mais longas

use std::io;

fn main() {
    let mut km:String = String::from("");

    println!("Qual a distância da viagem: ");
    io::stdin()
        .read_line(&mut km)
        .expect("Error ler km");
    let km:f32 = km.trim().parse().expect("Erro ao converter km");

    let valor = if km <= 200.00 { km * 0.50 } else { km * 0.45 };

    println!(
        "A distancia da viagem é de {} km. \nO preço fica em R${:.2} reais.",
        km, valor
    )
}