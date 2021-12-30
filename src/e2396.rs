use std::io;
use io::Read;
#[derive (Clone, Debug)]
struct Rank {carro :i32, tempo: i32}
pub fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    //ler todos os números  para o buffer até alcançar EOF
    stdin.read_to_end(&mut buffer).expect("err");
    let mut voltas = 0;
    let mut rank : Vec<Rank> = Vec::new();
    for (en,i) in String::from_utf8(buffer).unwrap().lines().enumerate() { 
        if en == 0 {
            // let carros = i.split_ascii_whitespace().nth(0).unwrap().parse::<i32>().unwrap() ; //não faz diferença, é lido tudo.
            voltas = i.split_ascii_whitespace().nth(1 as usize).unwrap().parse::<i32>().unwrap() ;
            continue;
        }
        let mut tempo = 0;
        for vlt in 0..voltas {
            tempo += i.split_ascii_whitespace().nth(vlt as usize).unwrap().parse::<i32>().unwrap();
        }
        rank.push(Rank{carro: en as i32, tempo: tempo} );
    }
    rank.sort_by_key(|m| m.tempo);
    for i in 0..3 {
        println!("{}",rank[i].carro);
    }
}