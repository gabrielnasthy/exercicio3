// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;

fn ler()->i32{
    let mut input=String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        input.trim().parse().unwrap()
}

fn main() {
    println!("digite um numero inteiro");
    let mut num1=ler();
    println!("digite outro numero inteiro");
    let mut num2=ler();

    if num1 > num2{
        println!("numero {} é o maior", num1);
    }else{
        println!("numero {} é o maior", num2);
    }

}
