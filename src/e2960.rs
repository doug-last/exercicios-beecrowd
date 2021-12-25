// beecrowd | 2960
// Era Uma Vez…
// Por Samuel Eduardo da Silva, IFSULDEMINAS/UFF BR Brazil
// Timelimit: 1
// Desde criança, histórias nos fascinam. Seja aventura, ficção científica, romance... independente do gênero, toda história, se bem contada, nos proporciona conhecimento, escape e muito divertimento.
// Muitas histórias nos são apresentadas em formato de livro, e este é um dos bens mais valiosos de nossas vidas. Infelizmente, muitas pessoas menosprezam livros, pois elas não têm conhecimento do quão importante essa mídia é.
// Cate é uma garota que adora leitura, e quanto mais livros consome, mais quer consumir. Com o tempo, adquiriu muitos, e tem quase uma biblioteca em casa. Certo dia, estava buscando formas de aproveitar ainda mais seus livros, e percebeu algo interessante: se pegasse a primeira letra de cada título, conseguia formar palavras interessantes. Então, decidiu pegar alguns livros de forma aleatória e, com a primeira letra do título de cada um, formar palavras.
// Após gerar várias palavras, decidiu que iria criar uma nova linguagem, baseada nestas palavras. O nome da linguagem seria o conjunto de letras iniciais das palavras geradas. Como é uma pessoa muito perfeccionista, gostaria de gerar, para sua nova linguagem, um relatório que contém: Qual a quantidade distinta de vogais e consoantes existem na sua linguagem. Ela também gosta de exatas (apesar de não ser muito boa), e propôs uma fórmula que dita quanto tempo uma pessoa demoraria para aprender essa nova linguagem. A fórmula é a seguinte: (número total de letras distintas + número de vogais) dividido pela quantidade total de consoantes. A métrica usada, para este valor, é horas. O resultado dessa fórmula também deve estar presente no relatório.
// Como o mais novo livro de Dão Brão, seu autor favorito, foi publicado, está muito ocupada lendo, mas pediu para você criar um programa, que dadas as seleções aleatórias de livros que fez, indique as palavras criadas e gere o relatório descrito acima.
// Entrada
// A primeira linha contém um inteiro N, indicando o número de seleções aleatórias que fez. As próximas N linhas descrevem as seleções. Cada seleção inicia com um inteiro M, indicando a quantidade de livros que escolheu para esta seleção. Cada uma das M linhas seguintes descreve o título de um livro, que é uma cadeia de caracteres, podendo conter letras maiúsculas, minúsculas e espaços.
// Limites: 1 ≤ N ≤ 100;
// 1 ≤ M ≤ 20;
// Cada nome de livro não supera 100 caracteres.
// Saída
// Deve conter o relatório apresentado na descrição do problema. A ordem das informações pode ser vista no caso de teste. O nome da linguagem deve ser apresentado em letras maiúsculas, e as palavras, em minúscula. O resultado da fórmula deve ser apresentado com uma casa decimal. Se o total de consoantes for nulo, em vez do total de horas, deve ser mostrado a mensagem “Linguagem Ruim”, no final do relatório.

use std::io;
use std::io::Read;
pub fn run() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    let mut nome_da_linguagem = String::from("");
    let mut lista_de_palavras = String::from("");
    let mut numero_de_vogais = 0;
    let mut numero_de_consoantes = 0;
    let mut numero_total_de_letras = 0;
    let mut vogais = String::from("EIOUA"); //pois é...
    let mut consoantes = String::from("BCDFGHJKLMNPQRSTVXZWY");   
    let mut aguardando_letra = false;

    stdin.read_to_end(&mut buffer).expect("ih");
    for i in String::from_utf8(buffer).unwrap().lines() {
        if !i.trim().parse::<i32>().is_err() {
            aguardando_letra = true;
            // println!("{:?}",i.parse::<i32>().unwrap());
            if lista_de_palavras.is_empty() || lista_de_palavras.chars().last().unwrap() != '\n' {
                lista_de_palavras.push('\n');
            }
        } 
        else if i.trim().chars().nth(0) != None {
            if aguardando_letra {
                nome_da_linguagem.push(i.trim().chars().nth(0).unwrap());
                aguardando_letra = false;
            }

            if vogais.rfind(i.trim().chars().nth(0).unwrap().to_ascii_uppercase()) != None  {
                 vogais.remove(vogais.rfind(i.trim().chars().nth(0).unwrap().to_ascii_uppercase()).unwrap());
                numero_de_vogais = numero_de_vogais +1;
                numero_total_de_letras = numero_total_de_letras+1;
            } 
            else if consoantes.rfind(i.trim().chars().nth(0).unwrap().to_ascii_uppercase()) != None  {
                consoantes.remove(consoantes.rfind(i.trim().chars().nth(0).unwrap().to_ascii_uppercase()).unwrap());
                numero_de_consoantes = numero_de_consoantes +1;
                numero_total_de_letras = numero_total_de_letras +1;
            } 
            lista_de_palavras.push(i.trim().chars().nth(0).unwrap()); 
        }
    }
    println!("Nome da Linguagem: {}", nome_da_linguagem.to_uppercase());
    println!("Lista de Palavras:{}", lista_de_palavras.to_lowercase());
    println!("Numero de Vogais: {}", numero_de_vogais);         
    println!("Numero de Consoantes: {}", numero_de_consoantes);
    println!("Numero Total de Letras: {}", numero_total_de_letras);
    println!("Tempo para aprender: {:.1} horas", (numero_total_de_letras+numero_de_vogais) as f32 /(numero_total_de_letras-numero_de_vogais) as f32 );
}
