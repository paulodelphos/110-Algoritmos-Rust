// Faça um programa que leia uma frase pelo teclado e mostre:
// -> Quantas vezes aparece a letra "A".
// -> Em que posição ela aparece a primeira vez.
// -> Em que posição ela aparece a últiva vez.
fn main(){
    let  nome:String = String::from("Clara Ana luiza");
    let nome = nome.trim().to_lowercase();
    
    println!("Nome: {}", nome);
    let quantidade = nome.matches("a").count();
    println!("Letra a aparece {} vezes", quantidade);
    
    //let count_nome = nome.find("a");
    let primeira = nome.find("a");
    println!("{:?}", primeira);

    let caracteres = nome.chars();

    let mut i = 0;
    let mut pos = 0;
    for v in caracteres {
        if v == 'a' {
            if pos == 0 {
                println!("Letra 'a' aperece na  PRIMEIRA vez na pos: {}.", pos = i)
             }
            pos = i;
           // println!("Deu match na possicão {}", pos);
        };
        i += 1;
    }
    println!("Letra 'a' na ÚLTIMA vez aparece na posição {}", pos);
    
    

}