// use crate::models::users::{ApiResponse, User, UserData};
// use crate::RustlyDatastore;
// use rocket::serde::json::Json;
// use rocket_db_pools::Connection;

// //the login page accessible only to unauthenticated users via /auth/login
// //to login a user, the user must provide a username and password
// // the data will be validated a session will begin to track the currently logged in user the user will be stored in the session and the corresponding token/user data will be returned to the user interface
// #[post("/login", format = "application/json", data = "<user>")]
// pub async fn login(
//     database: Connection<RustlyDatastore>,
//     user: Json<User>,
// ) -> Json<ApiResponse<UserData>> {
//     //TODO validate the user data  for standard compliance then validate the password
//     let user = user.into_inner();
//     let user_data = User::get_user(database, user.username, user.password).await;
//     // let user_data = user_data.await;
//     // let user_data = user_data.unwrap();
//     Json(ApiResponse {
//         success: true,
//         message: "Login successsfully".to_string(),
//         data: user_data,
//     })
// }

// //the sign up page accessible only to unauthenticated users via /auth/signup
// #[post("/signup", data = "<user>", format = "application/json")]
// pub async fn sign_up(
//     database: Connection<RustlyDatastore>,
//     user: Json<User>,
// ) -> Json<ApiResponse<()>> {
//     //TODO check if username or email already exist
//     let new_user = User::new(database, user.into_inner()).await;
//     Json(new_user)
// }
