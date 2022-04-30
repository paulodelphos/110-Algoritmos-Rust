// Faça um program que leia o ano qualquer e mostre se ele é BISSEXTO
use std::io;

fn main() {
    let mut ano:String = String::from("");

    println!("Que ano quer analisar? ");
    io::stdin()
        .read_line(&mut ano)
        .expect("Erro ao ler ano");
    let ano:i32 = ano.trim().parse().expect("Error ao converter ano");


   if ano % 4 == 0 && ano % 100 != 0 && ano % 400 == 0 {
       println!("O ano {} é BISSEXTO", ano);
   } else {
       println!("O ano {} NÃO é BISSEXTO", ano);
   }
       

}