#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate rocket;
use dotenv::dotenv;
use rocket::*;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}


fn main() {
    dotenv().ok();
    println!("Parsed environment variables");
    let url = dotenv!("DATABASE_URL");
    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let _conn = pool.get_conn().unwrap();
    println!("Successfully connected to PlanetScale!");
    // rocket::build().mount("/hello", routes![world]);
}



