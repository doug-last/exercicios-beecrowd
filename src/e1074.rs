use std::io;
use io::Read;
pub fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    //ler todos os números  para o buffer até alcançar EOF
    stdin.read_to_end(&mut buffer).expect("err");
    for i in String::from_utf8(buffer).unwrap().lines().skip(1) { //está pulando o primeiro elemento ao invés de ler
        if i.is_empty() {
            break;
        }
        let entrada = i.trim().parse::<i32>().unwrap();
        if entrada == 0 { //zero
            println!("NULL");
        }else if entrada < 0 { //negativos
            if entrada % 2 == 0 {
                println!("EVEN NEGATIVE");
            }
            else {
                println!("ODD NEGATIVE");
            }
        } else { //positivos
            if entrada % 2 == 0 {
                println!("EVEN POSITIVE");
            }
            else {
                println!("ODD POSITIVE");
            }  
        }
    }
}
