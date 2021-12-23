// Mariazinha sabe que um Número Primo é aquele que pode ser dividido somente por 1 (um) e por ele mesmo. Por exemplo, o número 7 é primo, pois pode ser dividido apenas pelo número 1 e pelo número 7 sem que haja resto. Então ela pediu para você fazer um programa que aceite diversos valores e diga se cada um destes valores é primo ou não. Acontece que a paciência não é uma das virtudes de Mariazinha, portanto ela quer que a execução de todos os casos de teste que ela selecionar (instâncias) aconteçam no tempo máximo de um segundo, pois ela odeia esperar.
// Entrada
// A primeira linha da entrada contém um inteiro N (1 ≤ N ≤ 200), correspondente ao número de casos de teste. Seguem N linhas, cada uma contendo um valor inteiro X (1 < X < 231) que pode ser ou não, um número primo.
// Saída
// Para cada caso de teste imprima a mensagem “Prime” (Primo) ou “Not Prime” (Não Primo), de acordo com o exemplo abaixo.

use std::io;
// use primes::{Sieve, Primeset};
//copiado do crate primes:
/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
    // TODO: return to step_by
    // for n in (3..).step_by(2).take_while(|m| m*m <= x) {
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    // No factor found. It must be prime.
    x
}
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}
pub fn run() {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("ent");
    for _i in 0..entrada.trim().parse::<i32>().unwrap()
    {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("loop");
        let mut resultado = {
            if is_prime(entrada.trim().parse::<u64>().unwrap()) 
            {    
                "Prime"
            }
            else { 
                "Not Prime"
            }
        };
        println!("{}", resultado);
    }
}
