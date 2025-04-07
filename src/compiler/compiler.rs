use crate::ast::*;
use crate::bytecode::Instrucao;

pub fn compilar(statements: vec<Statement>) -> Vec<Instrucao> {
    let mut bytecode = vec::new();

    for stmt in statements {
        match stmt {
            Statement::VarDecl { nome, valor, .. } => {
                match valor {
                    Expr::Texto(s) => {
                        bytecode.push(Instrucao::CarregarTexto(s));
                    }
                    Expr::Numero(n) => {
                        bytecode.push(Instrucao::CarregarNumero(n));
                    }
                    Expr::Ident(nome_var) => {
                        bytecode.push(Instrucao::LerVariavel(nome_var));
                    }
                }

                bytecode.push(Instrucao::SalvarVariavel(nome));
            }

            Statement::Exibir(expr) => {
                match expr {
                    Expr::Texto(s) => bytecode.push(Instrucao::CarregarTexto(s)),
                    Expr::Numero(n) => bytecode.push(Instrucao::CarregarNumero(n)),
                    Expr::Ident(nome_var) => bytecode.push(Instrucao::LerVariavel(nome_var))
                }

                bytecode.push(Instrucao::Exibir);
            }
        }
    }

    bytecode.push(Instrucao::fim);
    bytecode 
}