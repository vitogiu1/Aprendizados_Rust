use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
            let mut chances = 3;

    let mut modo_jogo = String::new();

    println!("Escolha o modo de jogo: (1) - Normal (2) - Adm (QUALQUER RESPOSTA INVÁLIDA, IRÁ TE RETIRAR DO JOGO!): ");

    io::stdin()
        .read_line(&mut modo_jogo)
        .expect("Falha ao ler o modo de jogo");

    if modo_jogo == "2\n" {
        println!("Bem vindo, ADM!");
        println!("O número secreto é {}", numero_secreto);
    } else {
        println!("Bem vindo, Jogador!");
    }

    loop {
        let mut palpite = String::new();

        if chances > 1 {
        println!("Você tem apenas {} chances! Digite o seu palpite ", chances);
        } else {
            println!("Você tem apenas 1 chance! Digite o seu palpite: ");
        } 
            io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler o palpite");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você colocou: {}", palpite);
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                chances -= 1; println!("Você errou! O número é maior! Você agora tem {}/3 chances", chances);
            }
            Ordering::Greater => {
                chances -= 1; println!("Você errou! O número é menor! Você agora tem {}/3 chances", chances);
            }
            Ordering::Equal => {
                println!("Você acertou! O número é {}!", numero_secreto);
                break;
            }
        }

            if chances == 0 {
            println!("Você perdeu! O número era {}!", numero_secreto);
            break;
        }
    }
}
