#[derive(Debug)]
pub enum Instrucao {
    CarregarTexto(String), // Push string
    CarregarNumero(i64),   // Push number
    SalvarVariavel(String),
    LerVariavel(String),
    Exibir,
    Fim,
}
