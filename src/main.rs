use std::io;

fn main() {
    println!("Olá mundo");
    println!("Este é meu primeiro app em Rust");
    println!("Versão 1.0");
    println!("");
    println!("Informe seu nome:");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Ocorreu um erro"); 

    println!("Seu nome é {}", name);
    println!("");

    println!("Pressione qualquer tecla para fechar a janela...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ocorreu um erro");   
}