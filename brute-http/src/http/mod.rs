use actix::Addr;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use get::{
    get_brute_attackers, get_brute_city, get_brute_country, get_brute_protocol, get_brute_region,
};
use log::info;
use post::{post_brute_attack_add, post_brute_protocol_increment};
use rustls::ServerConfig;

use crate::system::BruteSystem;

mod get;
mod post;

#[derive(Clone)]
pub struct AppState {
    actor: Addr<BruteSystem>,
    bearer: String,
}

pub async fn serve(
    ip: &str,
    port: u16,
    brute_actor: Addr<BruteSystem>,
    bearer_token: String,
) -> anyhow::Result<()> {
    info!("(TLS) Listening on {}:{}", ip, port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
            ])
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(AppState {
                actor: brute_actor.clone(),
                bearer: bearer_token.clone(),
            }))
            .wrap(cors)
            .service(
                web::scope("brute")
                    .service(post_brute_attack_add)
                    .service(post_brute_protocol_increment)
                    .service(get_brute_attackers)
                    .service(get_brute_protocol)
                    .service(get_brute_country)
                    .service(get_brute_city)
                    .service(get_brute_region),
            )
    })
    .bind((ip, port))
    .unwrap()
    .run()
    .await
    .unwrap();
    Ok(())
}

pub async fn serve_tls(
    ip: &str,
    port: u16,
    brute_actor: Addr<BruteSystem>,
    tls_config: ServerConfig,
    bearer_token: String,
) -> anyhow::Result<()> {
    info!("Listening on {}:{}", ip, port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
            ])
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(AppState {
                actor: brute_actor.clone(),
                bearer: bearer_token.clone(),
            }))
            .wrap(cors)
            .service(
                web::scope("brute")
                    .service(post_brute_attack_add)
                    .service(post_brute_protocol_increment)
                    .service(get_brute_attackers)
                    .service(get_brute_protocol)
                    .service(get_brute_country)
                    .service(get_brute_city)
                    .service(get_brute_region),
            )
    })
    .bind_rustls_0_23((ip, port), tls_config)?
    .run()
    .await
    .unwrap();
    Ok(())
}
