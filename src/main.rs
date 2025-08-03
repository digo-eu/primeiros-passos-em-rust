use std::io;
use rand::Rng;
use colored::Colorize;

/// Define um personagem e suas características
struct Personagem {
    nome: &'static str,
    cor: &'static str,
}

impl Personagem {
    /// Exibe uma mensagem associada a um personagem
    fn fala(&self, texto: String) {
        println!("{}{}", self.nome.to_owned() + ": ", texto.color(&*self.cor));
    }
}

/// Exibe uma lista de opções e retorna a opção selecionada através da linha de comando
fn dialog_prompt(options: Vec<String>) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i + 1, option);
    }
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Não estou me sentindo bem, Coelho Sabido...");
    match answer.trim().parse() {
        Ok(i) if (i <= options.len()) & (i > 0) => return  i,
        _ => {
            print!( " Opções válidas: 1 a {}\n", options.len());
            return dialog_prompt(options);
        },
    };
}

fn main() {
    let urso: Personagem = Personagem {
        nome: "Urso",
        cor: "green",
    };
    let esquilo: Personagem = Personagem {
        nome: "Esquilo",
        cor: "yellow",
    };
    let narrador: Personagem = Personagem {
        nome: "Narrador",
        cor: "blue",
    };
    let diavolo: Personagem = Personagem {
        nome: "Diabo",
        cor: "red",
    };

    // Gera um número inteiro aleatório entre 5 e 20 para o custo da fantasia 
    let custo: u32 = rand::rng().random_range(5..=20);
    
    // Calcula um preço com markup de 20% sobre o custo, arredondando para cima nos centavos
    let preco: f32 = (custo as f32 * 1.2 * 100.0).round() / 100.0;
    
    esquilo.fala("Oi, Coelho Sabido! Você chegou em boa hora. o Urso fez essa fantasia mas não sabe quantos cascalhitos cobrar. Você pode nos ajudar?".to_string());

    let resposta = dialog_prompt(vec![
        "Sim".to_string(),
        "Não".to_string(),
        "Não sei".to_string(),
    ]);

    match resposta {
        1 => urso.fala(
            "É assim, seu Sabido. Gastei ".to_string() + &custo.to_string() + " cascalhitos em materiais para fazer essa fantasia. A Úrsula me recomendou cobrar sempre 20% a mais do que gastei. Quanto devo cobrar então?",
        ),
        2 => {
            narrador.fala("Assim Coelho Sabido viveu triste e sem amigos até o fim dos seus dias. Fim.".to_string());
            return;
        },
        _ => {
            diavolo.fala("É tempo de decisão, Sabido. Todo coelho conhece as consequências. Faça a sua escolha.".to_string(),);
            return;
        }
    }

    loop {
        let mut numero: String = String::new();
        
        io::stdin()
            .read_line(&mut numero)
            .expect("Não estou me sentindo bem, Coelho Sabido...");

        let numero: f32 = match numero.trim().parse() {
            Ok(a) => a,
            Err(b) => {
                urso.fala(
                    "Esse valor é esquisito. Veja o que eu entendi:".to_string() + &numero.trim() + "E olha no que isso dá: " + &b.to_string(), 
                );
                continue;
            },
        };

        let diferenca: f32 = numero - preco;
        
        match diferenca {
            x if x > 5.0 => esquilo.fala("Ficou doidão, Coelho? O fio é dourado mas não é ouro!.".to_string()),
            x if x < -5.0 => esquilo.fala("Ficou doidão, Coelho? Assim você quebra a firma!.".to_string()),
            x if x < -0.01 => urso.fala("Sei não, seu Sabido. Acho que dá mais.".to_string()),
            x if x > 0.01 => urso.fala("Eu até queria, mas não tenho esse talento todo, seu Sabido.".to_string()),
            _ => {
                urso.fala("Hmmmmm... Esse é o nosso número. Muito bem, Coelho Sabido!".to_string());
                break;
            }
        }
    }
    narrador.fala("E assim o senhor Urso agradece a sua ajuda e te convida a participar da próxima fantasia.".to_string());
}