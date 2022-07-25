use rocket_dyn_templates::{context, Template};
//the login page accessible only to unauthenticated users via /auth/login
#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", context! { title:"404 - Not Found" })
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("500", context! { title:"500 - Internal Server Error" })
}
