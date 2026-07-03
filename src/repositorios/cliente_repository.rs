use crate::models::cliente::Cliente;
use sqlx::MySqlPool;

pub async fn listar(
    pool: &MySqlPool,
) -> Vec<Cliente> {

    sqlx::query_as::<_, Cliente>(
        "
        SELECT
            id,
            nome,
            cpf
        FROM clientes
        "
    )
    .fetch_all(pool)
    .await
    .unwrap()
}