use rocket_dyn_templates::{context, Template};
//the login page accessible only to unauthenticated users via /auth/login
#[catch(404)]
pub fn not_found() -> Template {
    Template::render("errors/404", context! { /* name:"drizzle"  */})
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("errors/500", context! { /* name:"drizzle"  */})
}
