#[derive(Debug, Clone)]
pub enum Tipo {
    Texto,
    Numero,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Texto(String),
    Numero(i64),
    Ident(String),
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        nome: String,
        tipo: Tipo,
        valor: Expr,
    },
    Exibir(Expr),
}
