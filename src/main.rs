mod application_config;
mod errors;
mod handlers;
mod models;
mod repository;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use application_config::ApplicationConfig;
use errors::Error;
use flexi_logger;
use log::info;
use repository::Repository;
use std::fs;

fn read_config(path: &str) -> Result<ApplicationConfig, Error> {
    let config_bytes = fs::read(path)?;

    Ok(toml::from_slice(&config_bytes)?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = read_config("./config.toml")?;
    let repository = Repository::new(&config.db).await?;

    flexi_logger::Logger::try_with_str(config.log_level)?.start()?;

    info!("Start server!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(repository.clone()))
            .service(web::resource("/register").route(web::post().to(handlers::user_register)))
            .service(
                web::scope("/list")
                    .route("my", web::get().to(handlers::get_my_list))
                    .service(
                        web::resource("")
                            .route(web::post().to(handlers::create_list))
                            .route(web::get().to(handlers::get_all_list_ids)),
                    )
                    .service(
                        web::scope("/{list_id}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(handlers::get_list))
                                    .route(web::delete().to(handlers::delete_list))
                                    .route(web::put().to(handlers::modify_entry_order)),
                            )
                            .service(
                                web::scope("/entry")
                                    .route("/", web::post().to(handlers::add_entry))
                                    .route("/{entry_id}", web::delete().to(handlers::delete_entry)),
                            ),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
