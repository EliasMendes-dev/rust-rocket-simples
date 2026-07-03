use rocket_dyn_templates::{Template, context};
use crate::utils::date_utils::get_current_year;

#[get("/")]
pub fn index() -> Template {
    let year = get_current_year();
    Template::render("home/index", context! { current: "home", year: year})
}