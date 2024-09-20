// storage.harveycoombs.com - Written by Harvey Coombs
use actix_web::{web, App, HttpServer};

mod files;
mod routes;

use crate::routes::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{group}/{id}/{index}", web::get().to(single))
            .route("/{group}/{id}", web::get().to(uploads))
            .route("/{group}/{id}", web::delete().to(delete))
            .route("/{group}/upload", web::post().to(upload))
    })
    .bind(("localhost", 81))?
    .run()
    .await
}