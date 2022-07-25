use crate::RustlyDatastore;
use nanoid::nanoid;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};
#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub original_url: String,
    pub url_id: String,
    pub total_views: u32,
}

///an object to return response  for each api request
/// the object will contain a success field which return true or false
/// the object will contain a message field which will contain a message if the request is not successful and more detailed information
/// finally the field will contain a data which is implemented as a generic field.
/// the data field will contain any data that need to be sent back to the client
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl Link {
    /// Accepts
    /// Minify the url and store it in the database
    /// return the minified url
    pub async fn minify(mut database: Connection<RustlyDatastore>, original_url: String) -> String {
        let url_id = nanoid!(6); // generate a random string of 6 characters
        let total_views = 0;
        //save the data
        let _query =
            sqlx::query("INSERT INTO links (url_id, original_url, total_views) VALUES (?, ?, ?)")
                .bind(url_id.clone())
                .bind(original_url)
                .bind(total_views)
                .execute(&mut *database)
                .await;
        let minified_url = format!("{}/{}", "APP_URL", url_id);

        // let minified_url = format!("{}/{}", dotenv!("APP_URL"), url_id);
        minified_url
    }

    /// redirect to the original url
    pub async fn _redirect(
        mut database: Connection<RustlyDatastore>,
        url_id: String,
    ) -> sqlx::mysql::MySqlRow {
        let original_url = sqlx::query("SELECT original_url FROM links WHERE url_id = ?")
            .bind(url_id)
            .fetch_one(&mut *database)
            .await
            .unwrap();
        original_url
    }
}
