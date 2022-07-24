// use crate::RustlyDatastore;
// use nanoid::nanoid;
// use rocket::serde::{Deserialize, Serialize};
// use rocket_db_pools::{sqlx, Connection};
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Link {
//     pub original_url: String,
//     pub url_id: String,
//     pub total_views: u32,
// }

// impl Link {
//     /// Accepts
//     /// Minify the url and store it in the database
//     /// return the minified url
//     pub async fn minify(mut database: Connection<RustlyDatastore>, original_url: String) -> String {
//         let url_id = nanoid!(6); // generate a random string of 6 characters
//         let total_views = 0;
//         //save the data
//         let _query =
//             sqlx::query("INSERT INTO links (url_id, original_url, total_views) VALUES (?, ?, ?)")
//                 .bind(url_id.clone())
//                 .bind(original_url)
//                 .bind(total_views)
//                 .execute(&mut *database)
//                 .await;
//                 let minified_url = format!("{}/{}", "APP_URL", url_id);

//         // let minified_url = format!("{}/{}", dotenv!("APP_URL"), url_id);
//         minified_url
//     }

//     // pub fn get_minified_url(url_id: String) -> String {
//     //     format!("{}/{}", dotenv!("APP_URL"), url_id)
//     // }

//     // pub fn get(url_id: String) -> Link {
//     //     Link {
//     //         raw_url: "https://www.rust-lang.org/en-US/".to_string(),
//     //         url_id,
//     //         total_views: 0,
//     //     }
//     // }

//     // pub fn deactivate(url_id: String) -> Link {
//     //     Link {
//     //         raw_url: "https://www.rust-lang.org/en-US/".to_string(),
//     //         url_id,
//     //         total_views: 0,
//     //     }
//     // }

//     // pub fn reactivated(url_id: String) -> Link {
//     //     Link {
//     //         raw_url: "https://www.rust-lang.org/en-US/".to_string(),
//     //         url_id,
//     //         total_views: 0,
//     //     }
//     // }
// }
