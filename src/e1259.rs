use std::io;
use io::Read;
pub fn main() {
    let mut entrada = String::new();
    let mut pares = Vec::new();
    let mut impares = Vec::new();
    io::stdin().lock().read_to_string(&mut entrada).expect("err");
    for i in entrada.lines().skip(1) { 
        if i.trim().parse::<i32>().unwrap() % 2 == 0 {
            pares.push(i.trim().parse::<i32>().unwrap())
        } else {
            impares.push(i.trim().parse::<i32>().unwrap())
        }
    }
    pares.sort();
    impares.sort();
    impares.reverse();
    for i in pares {
        println!("{}", i)
    }
    for i in impares {
        println!("{}", i)
    }
}
