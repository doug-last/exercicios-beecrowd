use std::io;
pub fn run() {
        /*
    Leia 3 valores reais (A, B e C) e verifique se eles formam ou não um triângulo. Em caso positivo, calcule o perímetro do triângulo e apresente a mensagem:
    Perimetro = XX.X
    Em caso negativo, calcule a área do trapézio que tem A e B como base e C como altura, mostrando a mensagem
    Area = XX.X
    Entrada
    A entrada contém três valores reais.
    Saída
    O resultado deve ser apresentado com uma casa decimal.
    */
    let mut a = [3.0,2.0,1.0];
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falhou leitura");
    let mut iter = entrada.trim().split_ascii_whitespace();
    for element in a.iter_mut() {
        *element = iter.next().unwrap().trim().parse::<f32>().unwrap(); 
    }

        /*Como saber se 3 retas podem formar um triângulo?
    Resultado de imagem para descobrindo se é trapezio e triangulo tendo 3 lados
    Não é necessário fazer as três somas para verificar a possibilidade de um triângulo existir. 
    Basta fazer a soma entre os dois lados menores. 
    Se a soma entre eles for maior que o terceiro lado, 
    então, a soma entre qualquer um deles e o terceiro lado (que é o maior) terá o mesmo resultado.
    */
    if a[0] >= a[1] && a[0] >= a[2] { //primeira posição maior q as outras duas
        if a[1]+a[2] > a[0] { // se a soma for maior então é triangulo
            calcula_perimetro(a[0], a[1], a[2]); 
        } else {
            calcula_area(a[0], a[1], a[2]);
        }
    } else if a[1] >= a[2] && a[1] >= a[0] { //segunda posição maior...
        if a[0]+a[2] > a[1] { // se a soma for maior então é triangulo
            calcula_perimetro(a[0], a[1], a[2]); 
        } else {
            calcula_area(a[0], a[1], a[2]);
        }        
    } else { //terceira posição maior...
        if a[1]+a[0] > a[2] { // se a soma for maior então é triangulo
            calcula_perimetro(a[0], a[1], a[2]); 
        } else {
            calcula_area(a[0], a[1], a[2]);
        }        
    } 
}

//O perímetro do triângulo corresponde a soma de todos os lados dessa figura plana.
fn calcula_perimetro(n1: f32, n2: f32, n3: f32) {
   println!("Perimetro = {:.1}", (n1 + n2 + n3));
}
// Para calcular a área de um trapézio qualquer, 
// somamos os comprimentos da base maior com o da base menor, 
// multiplicamos o resultado da soma pela altura do trapézio e dividimos o produto por dois.
fn calcula_area(n1: f32, n2: f32, n3: f32) {
    println!("Area = {:.1}", (n1 + n2) * n3 / 2.0 );
}