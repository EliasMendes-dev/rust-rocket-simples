use sqlx::MySqlPool;

pub async fn conectar() -> MySqlPool {

    MySqlPool::connect(
        "mysql://root:root@localhost/clientes_rust_db"
    )
    .await
    .unwrap()
}