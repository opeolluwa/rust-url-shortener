//the user account pages
use rocket_dyn_templates::{context, Template};
//the login page accessible only to unauthenticated users via /auth/login
#[get("/dashboard")]
pub fn dashboard() -> Template {
    Template::render("dashboard/index", context! { /* name:"drizzle"  */})
}
