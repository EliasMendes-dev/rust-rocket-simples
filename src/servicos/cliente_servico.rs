use crate::{
    dtos::cliente_dto::ClienteDto,
    models::cliente::Cliente,
    repositorios::cliente_repository,
};
use sqlx::MySqlPool;

pub async fn get_clientes(pool: &MySqlPool) -> Vec<Cliente> {
    cliente_repository::listar(pool).await
}

pub async fn get_cliente_por_id(
    pool: &MySqlPool,
    id: u32,
) -> Option<Cliente> {
    cliente_repository::get_cliente_por_id(pool, id).await
}

pub async fn atualizar_cliente(
    pool: &MySqlPool,
    id: u32,
    cliente: &ClienteDto,
) -> Result<(), sqlx::Error> {
    cliente_repository::atualizar_cliente(pool, id, cliente).await
}

pub async fn criar_cliente(
    pool: &MySqlPool,
    cliente: &ClienteDto,
) -> Result<(), sqlx::Error> {
    cliente_repository::criar_cliente(pool, cliente).await
}

pub async fn excluir_cliente(
    pool: &MySqlPool,
    id: u32,
) -> Result<(), sqlx::Error> {

    cliente_repository::excluir_cliente(pool, id).await
}