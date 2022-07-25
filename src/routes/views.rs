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
