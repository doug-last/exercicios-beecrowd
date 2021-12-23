// Faça um programa que leia 5 valores inteiros. Conte quantos destes valores digitados são pares e mostre esta informação.
// Entrada
// O arquivo de entrada contém 5 valores inteiros quaisquer.
// Saída
// Imprima a mensagem conforme o exemplo fornecido, indicando a quantidade de valores pares lidos.

use std::io;
pub fn run() {
    let mut pares:i8 = 0;
    for _i in  0..5 {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("...");
        let numero = entrada.trim().parse::<i8>().unwrap();
        if numero % 2 == 0 {
            pares=1+pares;
        }
    }
    println!("{} valores pares", pares);
}