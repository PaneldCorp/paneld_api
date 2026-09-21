use actix_web::web;

use crate::handlers::author_handlers::*;

pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(
        web::scope("/api")
            .route("/authors", web::get().to(get_all_authors))
            .route("/new", web::post().to(new_author))
            .route("/author/{id}", web::post().to(get_author_by_id))
            .route("/author/{name}", web::post().to(get_author_by_name))
    );
}
