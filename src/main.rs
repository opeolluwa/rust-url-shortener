#[macro_use]
extern crate rocket; // rocket is a framework for building web applications with Rust.
// #[macro_use]
// extern crate dotenv_codegen;
use rocket::fs::{relative, FileServer}; //for serving static files
// use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::Template; //for parsing of templates
use routes::*;

mod models;
mod routes; //import the route handlers from the routes module //import the models from the models module

//init database configuration
// #[derive(Database)]
// #[database("rustly_datastore")]
// pub struct RustlyDatastore(sqlx::MySqlPool);

#[launch]
async fn rocket() -> _ {
   
    rocket::build()
        //views
        .mount("/", routes![views::index, views::login, views::sign_up,])
        // .mount("/auth", routes![auth::login, auth::sign_up]) //auth routes through POST requests
        .mount("/dashboard", routes![users::dashboard]) //dashboard routes through GET requests
        // .mount("/api", routes![api::minify])
        .mount("/static", FileServer::from(relative!("public"))) //static files
        .attach(Template::fairing()) //template engines
        // .attach(RustlyDatastore::init()) //database connection
        .register("/*", catchers![errors::not_found, errors::internal_error]) //register the application
}
