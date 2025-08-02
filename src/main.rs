use std::{cmp::Ordering, io};
use rand::Rng;
use colored::Colorize;

struct Personagem {
    nome: String,
    cor: dyn Colorize,
}

fn fala(personagem: Personagem, texto: String) {
    let cor = personagem.cor;
    print!("{}", texto)
}

fn main() {
    let segredo: u32 = rand::rng().random_range(1..=10);
    
    loop {
        println!(
            "Urso: {}",
            "Precisamos saber quanto dá essa brincadeira. Me dê um número, por favor. E ignora os centavos.".green()
        );

        let mut numero: String = String::new();
        
        io::stdin()
            .read_line(&mut numero)
            .expect("Não estou me sentindo bem, Coelho Sabido...");

        let numero: u32 = match numero.trim().parse() {
            Ok(a) => a,
            Err(b) => {
                println!(
                    "Urso: {} {} {} {}",
                    "Esse valor é esquisito. Veja o que eu entendi:".green(),
                    numero.trim(),
                    "E olha no que isso dá: ".green(),
                    b,
                );
                continue;
            },
        };

        if numero > 10 || numero < 1 {
            println!(
                "Esquilo: {}",
                "Está mais pra algo de 1 a 10, Coelho Sabido.".red(),
            );
        };

        match numero.cmp(&segredo) {
            Ordering::Less => println!(
                "Urso: {}{}{}",
                "O seu número é ".green(),
                numero,
                ". Adoro esse número! Mas é pequenininho demais, coitado.".green(),
            ),
            Ordering::Greater => println!(
                "Urso: {}{}{}",
                "O seu número é ".green(),
                numero,
                ". Adoro esse número! Mas relaxa, não é tudo isso.".green(),
            ),
            Ordering::Equal => {
                println!(
                    "Urso: {}",
                    "Hmmmmm... Esse é o nosso número. Muito bem, Coelho Sabido!".green(),
                );
                break;
            }
        }
    }
}