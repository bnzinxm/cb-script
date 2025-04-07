mod parser;
use std::fs;

fn main() {
    let codigo = fs::read_to_string("src/tests/test.cb")
        .expect("Erro ao ler o arquivo")
        .replace("\r\n", "\n");

    parser::parse_code(&codigo);
}
