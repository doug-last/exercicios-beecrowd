// Robbie é um robô muito carismático, e uma das coisas que ele mais gosta de fazer, além de brincar com Glória, é colecionar moedas. 
// Robbie possui várias moedas com valores iguais ou diferente, e de mesmo mesmo tamanho. 
// e elas são guardadas de maneira organizada uma sobre a outra dentro de um cilindro de vidro. 
// Robbie sempre faz um joguinho com Glória usando suas moedas quando ela pede pra brincar com ele de esconde-esconde, 
// ou quando ela pede pra ele levá-la para passear. 
// O jogo acontece da seguinte maneira: Glória escolhe um número N que será o salto das moedas que serão somadas, 
// então a cada Nmoedas o valor Vi da moeda é somado até que não haja mais moedas, ou seja, Σ de ((VM-(N*0))+(VM-(N*1))+(VM-(N*2) )...), 
// M é o número de moedas. Por exemplo, se existirem 5 moedas com os valores 1, 2 , 3, 4 e 5, 
// e Glória escolher 2 como valor do salto, então serão somadas as moedas 5, 3 e 1, resultando em 9, 
// ao final Robbie verifica se a soma dessas moedas é um número primo, se isso acontecer ele faz o que a Glória quer, 
// caso contrário, a garotinha convence Robbie a jogar novamente, 
// pois ela sempre consegue convencer ele de tudo, alegando que deixará de contar histórias pra ele, caso ele não faça a vontade dela.
// Você como um bom programador da U.S. Robots, ajudará esses dois amigos, escrevendo um programa irá dizer o resultado do jogo.
// Entrada
// A entrada contém vários casos de teste. A primeira linha de um caso de teste contém um inteiro M (2 ≤ M ≤ 20 ) que representa a quantidade de moedas. 
// Cada uma das próximas M linhas contém um inteiro Vi (1 ≤ Vi ≤ 500) que representa o valor da moeda Mi , 
// e por último um inteiro N (1 ≤ N ≤ M) que é o salto na soma escolhido por Glória.
// A entrada termina em EOF.
// Saída
// Imprima “You’re a coastal aircraft, Robbie, a large silver aircraft.”, caso Glória ganhe o jogo, ou 
// “Bad boy! I’ll hit you.”, caso Glória não ganhe o jogo. A saída não deve conter aspas.

//stackoverflow:
// The EOT byte (0x04) is used to this day by unix tty terminals to indicate end of input. 
// You type it with a Ctrl + D (ie. ^D) to end input to shells or any other program reading from stdin.
// However, as others have pointed out, this is distinct from EOF, which is a condition rather than a piece of data per se.

use std::io;
use io::Read;
use std::convert::TryInto;
pub fn run() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    let mut qtde_moedas = 0;
    let mut moedas = Vec::<i32>::new();
    //ler todos os números  para o buffer até alcançar EOF
    stdin.read_to_end(&mut buffer).expect("err");
    for i in String::from_utf8(buffer).unwrap().lines() {
        //primeira lida é numero de moedas
        if i.trim().parse::<i32>().is_err()
        {
            //println!("vazio!!")
        }
        else 
        {
            if qtde_moedas == 0 {
                qtde_moedas = i.trim().parse::<i32>().unwrap();   
            } else 
            {
                //coletando valor das moedas
                if qtde_moedas > moedas.len().try_into().unwrap()  {
                    moedas.push(i.trim().parse::<i32>().unwrap());
                }
                else //coletando "passo"
                {
                    let passo = i.parse::<i32>().unwrap();
                    //confere o jogo aqui !!!!
                    if conferir_jogo(moedas, passo) {
                        println!("You’re a coastal aircraft, Robbie, a large silver aircraft.");
                    } else {
                        println!("Bad boy! I’ll hit you.");
                    }
                    qtde_moedas= 0;
                    moedas = Vec::<i32>::new();
                }
            }
        }
    }
}
fn conferir_jogo(  valor_moedas:Vec<i32> , salto: i32) -> bool {
    //quantidade de moedas são posições no vetor .
    //último número: passo para conferir número primos..
    let mut soma:i32=0;
    for i in valor_moedas.iter().rev().step_by(salto.try_into().unwrap()).rev() 
    {
        soma+=i;
    }
    is_prime(soma.try_into().unwrap())
}

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
