use std::io;
pub fn run() {
    // Leia a hora inicial, minuto inicial, hora final e minuto final de um jogo. A seguir calcule a duração do jogo.
    // Obs: O jogo tem duração mínima de um (1) minuto e duração máxima de 24 horas.
    // Entrada
    // Quatro números inteiros representando a hora de início e fim do jogo.
    // Saída
    // Mostre a seguinte mensagem: “O JOGO DUROU XXX HORA(S) E YYY MINUTO(S)” .
    
    let mut entrada = String::new();
    let mut minutos:i16 ;
    io::stdin().read_line(&mut entrada).expect("Falhou leitura");
    let mut iter = entrada.trim().split_ascii_whitespace();

    //inicial:
    minutos = 60 * iter.next().unwrap().trim().parse::<i16>().unwrap() + iter.next().unwrap().trim().parse::<i16>().unwrap(); 
    //subtraindo inicial do final e convertendo para horas e minutos
    minutos = -1 * (minutos - (60 * iter.next().unwrap().trim().parse::<i16>().unwrap() + iter.next().unwrap().trim().parse::<i16>().unwrap() ));
    if minutos <= 0 {
        minutos = minutos + 24*60; //caso passe de meia noite precisa virar o dia.
    }
    println!("O JOGO DUROU {0} HORA(S) E {1} MINUTO(S)", minutos/60, minutos-(minutos/60)*60)
 
}
