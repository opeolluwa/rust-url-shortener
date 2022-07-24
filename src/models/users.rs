// use rocket::serde::{json::Json, Deserialize, Serialize};
// use rocket_db_pools::{sqlx, sqlx::Executor, Connection};
// // extern crate bcrypt;
// use crate::RustlyDatastore;
// // use bcrypt::{hash, DEFAULT_COST};

// ///define a public struct to handle  user data
// /// the object will be major interface in creation and query of users
// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub firstname: String,
//     pub lastname: String,
//     pub username: String,
//     pub password: String,
//     pub email: String,
// }

// ///an object to return response  for each api request
// /// the object will contain a success field which return true or false
// /// the object will contain a message field which will contain a message if the request is not successful and more detailed information
// /// finally the field will contain a data which is implemented as a geeric field.
// /// the data field will contain any data that need to be sent back to the client
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ApiResponse<T> {
//     pub success: bool,
//     pub message: String,
//     pub data: T,
// }

// ///bearer token for authentication
// // pub struct BearerToken {
// //     token: String,
// // }

// ///a struct to handle user data
// /// the object will be major interface in mutation and query of users
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserData {
//     pub firstname: String,
//     pub lastname: String,
//     pub username: String,
//     pub email: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ApiError<T> {
//     success: bool,
//     message: String,
//     data: T,
//     status_code: u16,
// }

// /// an object to return the status of a database operation
// /// the object will contain a success field which return true or false
// /// the object will also contain a data field which is implemented as a generic.
// /// the data field will hold and data that need to be sent back to the client application
// #[derive(Debug, Serialize, Deserialize)]
// pub struct DatabaseResponse<T> {
//     success: bool,
//     data: T,
// }

// /// methods to create and query the user object
// impl User {
//     pub async fn new(mut database: Connection<RustlyDatastore>, user: User) -> ApiResponse<()> {
//         // let hashed_password = hash(user.password, DEFAULT_COST).unwrap();
//         let hashed_password = user.password.clone();
//         let query =
//             sqlx::query("INSERT INTO users_information (firstname, lastname, username, password, email) VALUES (?, ?, ?, ?,?)")
//                 .bind(user.firstname)
//                 .bind(user.lastname)
//                 .bind(user.username)
//                 .bind(hashed_password)
//                 .bind(user.email);
//         //execute the query
//         let query_response = database.execute(query).await;
//         println!("{:?}", query_response);
//         //return the response
//         ApiResponse {
//             success: true,
//             data: (),
//             message: "User successfully created".to_string(),
//         }
//     }

//     /// retrieve a user from the database
//     pub async fn get_user(
//         mut database: Connection<RustlyDatastore>,
//         username: String,
//         _password: String,
//     ) -> UserData {
//         let query = sqlx::query("SELECT * FROM users_information WHERE username = ?")
//             .bind(username)
//             .fetch_one(&mut *database)
//             .await;

//         // let query_response = query.fetch_one(database);
//         println!("{:?}", Json::from(query));
//         UserData {
//             firstname: "".to_string(),
//             lastname: "".to_string(),
//             username: "".to_string(),
//             email: "".to_string(),
//         }
//     }
// }
