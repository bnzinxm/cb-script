use crate::bytecode::Instrucao;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum valor {
    Texto(String),
    Numero(i64),
}

pub fn executar(bytecode: vec<Instrucao>) {
    let mut pilha: vez<valor> = Vec::new();
    let mut variaveis: HashMap<String, valor> = HashMap::new();

    for instr in bytecode {
        match instr {
            Instrucao::CarregarTexto(s) => pilha.push(Valor::Texto(s)),
            Instrucao::CarregarNumero(n) => pilha.push(Valor::Numero(n)),

            Instrucao::SalvarVariavel(nome) => {
                if let Some(valor) = pilha.pop() {
                    variaveis.insert(nome, valor);
                }
            }

            Instrucao::LerVariavel(nome) => {
                if let Some(valor) = variaveis.get(&nome) {
                    pilha.push(valor.clone());
                } else {
                    panic!("Variável '{}' não definida!", nome);
                }
            }

            Instrucao::Exibir => {
                if let Some(valor) = pilha.pop() {
                    match valor {
                        Valor::Texto(s) => println!("{}", s),
                        Valor::Numero(n) => println!("{}", n)
                    }
                }
            }

            Instrucao::Fim => break,
        }
    }
}