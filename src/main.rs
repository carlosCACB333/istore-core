use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use istore::controllers::products;
use istore::controllers::{app, category};
use istore::utils::db::get_connection_pool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stage = env::var("STAGE").unwrap_or("development".to_string());
    if stage == "development" {
        dotenv().ok();
    }
    let port = u16::from_str_radix(&std::env::var("APP_PORT").unwrap(), 10).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    HttpServer::new(move || {
        log::info!("Starting server on port {}", port);

        let api = web::scope("/api")
            .service(products::routes())
            .service(category::routes());
        let front_url = env::var("FRONT_URL").unwrap_or_default();
        let cors = Cors::default()
            .allowed_origin("https://ipdf.lat")
            .allowed_origin(&front_url)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(get_connection_pool()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(api)
            .service(app::routes())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
