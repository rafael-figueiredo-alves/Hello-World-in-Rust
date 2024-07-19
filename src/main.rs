use std::io;

fn main() {
    println!("Olá mundo");
    println!("Este é meu primeiro app em Rust");
    println!("Versão 1.1.0");
    println!("");
    println!("Informe seu nome:");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Ocorreu um erro"); 

    print!("");
    println!("Informe sua idade:");

    let mut age: String = String::new();
    io::stdin().read_line(&mut age).expect("Idade é inválida");

    println!("Seu nome é {} e você tem {} anos", name, age);
    println!("");

    press_enter_to_close_window();
}

fn press_enter_to_close_window() {
    println!("Pressione qualquer tecla para fechar a janela...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ocorreu um erro"); 
}