use std::io;
use io::Read;
pub fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    //ler todos os números  para o buffer até alcançar EOF
    stdin.read_to_end(&mut buffer).expect("err");
    let mut maior = 0;
    let mut maiori = 0;
    for (iteracao, j) in String::from_utf8(buffer).unwrap().lines().enumerate() { //está pulando o primeiro elemento ao invés de ler
    //    println!("{:?} {:?}",iteracao,j.trim().parse::<i32>().unwrap());
        if j.trim().parse::<i32>().unwrap() > maior {
            maior = j.trim().parse::<i32>().unwrap();
            maiori = iteracao;
        }
    }
    println!("{}\n{}",maior,maiori+1);
}