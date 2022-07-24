use chrono::Datelike;
use rocket_dyn_templates::{context, Template};
//the home page
#[get("/")]
pub fn index() -> Template {
    let current_year = chrono::Utc::now().year();
    Template::render(
        "index",
        context! { title:"Rustly - minimal URL shortener", current_year  },
    )
}

//the login page accessible only to unauthenticated users via /auth/login
#[get("/login")]
pub fn login() -> Template {
    Template::render("login", context! { /* name:"drizzle"  */})
}

//the sign up page accessible only to unauthenticated users via /auth/signup
#[get("/signup")]
pub fn sign_up() -> Template {
    Template::render("signup", context! { /* name:"drizzle"  */})
}
