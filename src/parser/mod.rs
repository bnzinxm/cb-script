use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct CBParser;

pub fn parse_code(code: &str) {
    println!(">>> Código a ser parseado:\n{}", code);  // Debug

    match CBParser::parse(Rule::program, code) {
        Ok(pairs) => {
            for pair in pairs {
                println!("Comando reconhecido: {:?}", pair);
            }
        }
        Err(e) => {
            println!("Erro de parsing: {}", e);
        }
    }
}
