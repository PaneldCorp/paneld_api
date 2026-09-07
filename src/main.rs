use actix_web::{App, HttpServer};

use crate::{
    config::database::Database,
    utils::log::{Level, log}
};

mod config;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::init().await;

    log(Level::Info, format!("API serveur running on http://{}.", database.get_binding()).as_str());

    HttpServer::new(||
        {
            App::new()
        }
    )
    .bind(database.get_binding())?
    .run()
    .await
}
