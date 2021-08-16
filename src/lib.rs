use std::{env};
use std::error::Error;
use std::fs;
pub struct ARGSPARSER{

    pub query: String,
    pub filename: String

}


impl ARGSPARSER{
    
    pub fn new() -> Result<ARGSPARSER,String>{

        let args = env::args().collect::<Vec<String>>();

        if args.len() <3 {

            return Err(String::from("Passe o texto de busca e o nome do arquivo!"))

        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let result = ARGSPARSER{query:query,filename:filename};

        Ok(result)
    }

}

fn ler_arquivos(dir:&String) -> Result<String,Box<dyn Error>>{

    let texto = fs::read_to_string(dir)?;

    Ok(texto)

}   

pub fn run(args:ARGSPARSER) -> Result<(),Box<dyn Error>>{

    let texto = ler_arquivos(&args.filename)?;


    let linhas = procura(&args.query,&texto);

    println!("Linha encontradas :\n");

    for linha in linhas{
        println!("{}",linha)
    }

    Ok(())

}

pub fn procura<'a>(query:&'a str,texto:&'a str) -> Vec<&'a str>{

    let mut result = Vec::new();

    for linha in texto.lines(){

        if linha.contains(query){

            result.push(linha);

        }

    }

    result

}
