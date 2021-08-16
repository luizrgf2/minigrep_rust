use std::{process};
use minigrep_rust::{ARGSPARSER,run};



fn main() {
    //variaveis
    let args = ARGSPARSER::new().unwrap_or_else(|err|{

        println!("Passe todos os argumentos {}",err);
        process::exit(1);

    });
    
    //prints
    println!("Procurando por {}", &args.query);
    println!("No arquivo {}",&args.filename);
    
    if let Err(e) = run(args){

        println!("Erro na aplicação {}", e);
        process::exit(1);
    }

}

