mod parser;
use std::fs;

fn main() {
    let input = r#"
        var nome: texto = "CB Script"
        exibir nome
    "#;

    let parsed = CBScriptParser::parse(Rule::program, input)
        .unwrap()
        .next()
        .unwrap();

    let ast = transformar_para_ast(parsed);
    let bytecode = compilar(ast);

    executar(bytecode);
}
