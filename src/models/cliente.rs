use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}
