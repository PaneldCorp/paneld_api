use actix_web::{
    web::Data,
    App,
    HttpServer
};

use crate::{
    config::{
        database::Database,
        services::Services
    },

    utils::log::{Level, log}
};

mod config;
mod models;
mod services;
mod repositories;
mod handlers;
mod routes;
mod utils;

pub struct AppState
{
    pub services: Services
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::init().await;
    let services = Services::new(database.clone().pool);
    let app_state = Data::new(AppState{ services });

    log(Level::Info, format!("API serveur running on http://{}.", database.get_binding()).as_str());

    HttpServer::new(move ||
        {
            App::new()
                .app_data(app_state.clone())
                .configure(routes::author_routes::config)
        }
    )
    .bind(database.get_binding())?
    .run()
    .await
}
