use crate::repositorios::cliente_repository;
use crate::models::cliente::Cliente;
use sqlx::MySqlPool;

pub async fn get_clientes(pool: &MySqlPool) -> Vec<Cliente> {
    cliente_repository::listar(pool).await
}
