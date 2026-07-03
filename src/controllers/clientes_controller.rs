use crate::{servicos::cliente_servico::get_clientes};
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
pub fn criar(cliente_dto_form: Form<ClienteDto>) -> Result<Redirect, Flash<Redirect>> {

    let cliente_dto = cliente_dto_form.into_inner();

    // simulando - salvar os dados no banco de dados
    println!("Nome do cliente: {}", cliente_dto.nome);
    println!("CPF do cliente: {}", cliente_dto.cpf);

    if 1 == 1 {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to("/clientes/novo"), 
            "Erro ao cadastrar clientes",
        ))
    }
}
