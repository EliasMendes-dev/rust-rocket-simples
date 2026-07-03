use crate::servicos::cliente_servico::{
    get_clientes,
    get_cliente_por_id,
    criar_cliente,
    atualizar_cliente,
};
use crate::utils::date_utils::get_current_year;
use rocket::request::FlashMessage;
use rocket::{form::Form, response::{Flash, Redirect}};
use rocket_dyn_templates::{Template, context};
use crate::dtos::cliente_dto::ClienteDto;
use rocket::State;
use sqlx::MySqlPool;

#[get("/clientes")]
pub async fn index(pool: &State<MySqlPool>) -> Template {
    
    let clientes = get_clientes(pool.inner()).await;

    Template::render(
        "clientes/index",
        context! { clientes: &clientes, current: "clientes", year: get_current_year(), },
    )
}

#[get("/clientes/novo")]
pub fn novo(flash: Option<FlashMessage<'_>>) -> Template {

    let mensagem = flash.map(|f| f.message().to_string());

    Template::render(
        "clientes/novo",
        context! {
            current: "clientes",
            year: get_current_year(),
            flash: mensagem,
        },
    )
}

#[post("/clientes/criar", data = "<cliente_dto_form>")]
pub async fn criar(
    cliente_dto_form: Form<ClienteDto>,
    pool: &State<MySqlPool>,
) -> Result<Redirect, Flash<Redirect>> {

    let cliente_dto = cliente_dto_form.into_inner();

    match criar_cliente(pool.inner(), &cliente_dto).await {
        Ok(_) => Ok(Redirect::to("/clientes")),
        Err(_) => Err(Flash::error(
            Redirect::to("/clientes/novo"),
            "Erro ao cadastrar cliente",
        )),
    }
}

#[get("/clientes/<id>/editar")]
pub async fn editar(
    id: u32,
    pool: &State<MySqlPool>,
    flash: Option<FlashMessage<'_>>,
) -> Template {

    let mensagem = flash.map(|f| f.message().to_string());

    let cliente = get_cliente_por_id(pool.inner(), id).await;

    Template::render(
        "clientes/editar",
        context! {
            cliente: cliente,
            flash: mensagem,
            current: "clientes",
            year: get_current_year(),
        },
    )
}

#[post("/clientes/<id>/editar", data = "<cliente_form>")]
pub async fn atualizar(
    id: u32,
    pool: &State<MySqlPool>,
    cliente_form: Form<ClienteDto>,
) -> Result<Redirect, Flash<Redirect>> {

    let cliente = cliente_form.into_inner();

    match atualizar_cliente(pool.inner(), id, &cliente).await {
        Ok(_) => Ok(Redirect::to("/clientes")),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/clientes/{}/editar", id)),
            "Erro ao atualizar cliente",
        )),
    }
}