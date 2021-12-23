// Escreva um algoritmo que leia 2 números e imprima o resultado da divisão do primeiro pelo segundo. Caso não for possível mostre a mensagem “divisao impossivel” para os valores em questão.
// Entrada
// A entrada contém um número inteiro N. Este N será a quantidade de pares de valores inteiros (X e Y) que serão lidos em seguida.
// Saída
// Para cada caso mostre o resultado da divisão com um dígito após o ponto decimal, ou “divisao impossivel” caso não seja possível efetuar o cálculo.
// Obs.: Cuide que a divisão entre dois inteiros em algumas linguagens como o C e C++ gera outro inteiro. Utilize cast :)
use std::io;
pub fn run() {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    for _num in 0..entrada.trim().parse().unwrap() {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let mut divida = entrada.trim().split_ascii_whitespace();
        let num: f32 = divida.next().unwrap().trim().parse::<f32>().unwrap();
        let num2: f32 = divida.next().unwrap().trim().parse::<f32>().unwrap();
        if num2 == 0.0 {
            println!("divisao impossivel");
        } else {
            println!("{:?}", num/num2);
        }
    }
}
