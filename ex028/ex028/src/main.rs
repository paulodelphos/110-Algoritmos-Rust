use std::io;
use rand::Rng;

fn main() {
    

    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("Vou pensar em um número entre 0 e 5. Tente adivinhar..");
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("Em que número eu pensei? ");
    
    loop {
        let mut chute:String = String::from("");
        io::stdin()
            .read_line(&mut chute)
            .expect("Erro ao ler chute");
        let chute:u32 = chute.trim().parse().expect("Error na conversao de chute");

        let aleatorio = rand::thread_rng().gen_range(1..6);
        
        if chute == aleatorio {
            println!("PARABENS! Eu também pensei {}. Você acertou!!", aleatorio);
            break;
        }else {
            println!("GANHEI! Eu pensei no número {} e não no {}", aleatorio, chute);
            println!("Tente novamente, em que númerou estou pensando?");
        }
    }
}
