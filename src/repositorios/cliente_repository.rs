use crate::{dtos::cliente_dto::ClienteDto, models::cliente::Cliente};
use sqlx::MySqlPool;

pub async fn listar(pool: &MySqlPool) -> Vec<Cliente> {
    sqlx::query_as::<_, Cliente>(
        "
        SELECT
            id,
            nome,
            cpf
        FROM clientes
        ",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn get_cliente_por_id(pool: &MySqlPool, id: u32) -> Option<Cliente> {
    sqlx::query_as::<_, Cliente>(
        "
        SELECT
            id,
            nome,
            cpf
        FROM clientes
        WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap()
}

pub async fn atualizar_cliente(
    pool: &MySqlPool,
    id: u32,
    cliente: &ClienteDto,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        UPDATE clientes
        SET
            nome = ?,
            cpf = ?
        WHERE id = ?
        ",
    )
    .bind(&cliente.nome)
    .bind(&cliente.cpf)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn criar_cliente(
    pool: &MySqlPool,
    cliente: &ClienteDto,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO clientes (nome, cpf)
        VALUES (?, ?)
        ",
    )
    .bind(&cliente.nome)
    .bind(&cliente.cpf)
    .execute(pool)
    .await?;

    Ok(())
}
