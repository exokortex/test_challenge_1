use std::collections::HashMap;
use std::time::Duration;

use actix_cors::Cors;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{App, error, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::http::Error;
use futures::future::{ready, Ready};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;

lazy_static! {
    static ref LOOKUP: HashMap<&'static str, &'static str> = {
        let mut lookup = HashMap::new();
        lookup.insert("922FE339AE85E0950EBA469674AE71411F10FCC2C0626A9E52C46772A04D31A9", "110001 110100 110111 110000 110001 110110 110001 110111 110011 1000100 110000 1000011 1000100 110110 110110 110010 110011 110001 1000001 110001 1000110 110100 1000001 110011 111000 110100 110000 110101 1000001 1000010 1000010 111001 110010 110011 1000011 1000001 1000101 1000010 1000101 1000010 110110 110001 110100 110101 111001 1000010 110110 1000100 110101 1000101 110111 111000 1000001 1000101 110110 110100 1000010 1000101 1000001 111001 1000100 111001 1000110 111000");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAF", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAG", "redacted");
        lookup.insert("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH", "KDCTF{here_would_be_the_flag}");
        lookup
    };
}

#[derive(Serialize, Deserialize)]
struct Msg {
    msg: String,
    err: bool,
}

impl Responder for Msg {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index(json_msg: web::Json<Msg>) -> impl Responder {
    if LOOKUP.contains_key(&*json_msg.msg) {
        return Msg { msg: LOOKUP[&*json_msg.msg].to_string(), err: false };
    }
    return Msg { msg: "The moment when the user just inserts some wrong input.".to_string(), err: true };
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = MemoryStore::new();
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        let cors = Cors::permissive();

        App::new().service(
            web::resource("/")
                .wrap(
                    RateLimiter::new(
                        MemoryStoreActor::from(store.clone()).start())
                        .with_interval(Duration::from_secs(10))
                        .with_max_requests(3)
                )
                .wrap(cors)
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(index)),
        )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
