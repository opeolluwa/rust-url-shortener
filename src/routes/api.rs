// use crate::models::link::Link;
// use crate::models::users::ApiResponse;
// use crate::RustlyDatastore;
// use rocket::serde::{json::Json, Deserialize, Serialize};
// use rocket_db_pools::Connection;
// #[derive(Deserialize, Debug, Serialize, Clone)]
// pub struct RequestPayload {
//     pub original_url: String,
// }

// #[post("/v1/links/minify", format = "application/json", data = "<payload>")]
// pub async fn minify(
//     database: Connection<RustlyDatastore>,
//     payload: Json<RequestPayload>,
// ) -> Json<ApiResponse<String>> {
//     let minified_url = Link::minify(database, payload.original_url.to_string()).await;
//     Json(ApiResponse {
//         success: true,
//         message: "Minified url successfully".to_string(),
//         data: minified_url,
//     })
// }
