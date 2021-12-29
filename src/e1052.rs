use std::io;
pub fn main() {
    let mut entrada = String::new();
    let meses = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]; 
    io::stdin().read_line(&mut entrada).unwrap();
    println!("{}", meses[(entrada.trim().parse::<usize>().unwrap()-1)]);
}