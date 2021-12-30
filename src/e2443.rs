use std::io;
use io::Read;

pub fn main () {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).expect("err");
    let n1 = entrada.split_ascii_whitespace().nth(0).unwrap().parse::<i32>().unwrap() ;
    let n2 = entrada.split_ascii_whitespace().nth(1).unwrap().parse::<i32>().unwrap() ;
    let n3 = entrada.split_ascii_whitespace().nth(2).unwrap().parse::<i32>().unwrap() ;
    let n4 = entrada.split_ascii_whitespace().nth(3).unwrap().parse::<i32>().unwrap() ;      
    for i in (1..(n4*n2) as i32 +1).rev()  {
        let div = (n2*n4); //divisor final
        let num = (div/n2*n1) + (div/n4*n3) ;//numerador final
        if num % i == 0 && div % i == 0
        { 
            println!("{} {}",num/i, div/i );
            break;
        }
    }
}