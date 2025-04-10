use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use chrono::Utc;
use log::{error, info};
use serde::Serialize;
use std::{env, io};
use uuid::Uuid;

#[derive(Serialize)]
struct UUIDv4Response {
    uuid: String,
    created_at: String,
    ip_address: String,
    user_agent: String,
}

#[get("/generate-uuid-v4")]
async fn generate_uuid_v4(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {
    let ip: String = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("Unknown IP")
        .to_string();

    info!("Received request from IP: {}", ip);

    let user_agent: String = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown User-Agent")
        .to_string();

    info!("User-Agent: {}", user_agent);

    let created_at = Utc::now();
    let uuid = Uuid::new_v4();

    info!("Generated UUIDv4: {} at {}", uuid, created_at);

    Ok(HttpResponse::Ok().json(UUIDv4Response {
        uuid: uuid.to_string(),
        created_at: created_at.to_rfc3339(),
        ip_address: ip,
        user_agent: user_agent,
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or("6969".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    let workers = num_cpus::get_physical();

    info!(
        "Starting server with {} workers on {}:{}",
        workers, host, port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(web::scope("/api/v1").service(generate_uuid_v4))
            .app_data(actix_web::web::Data::new(workers))
    })
    .workers(workers)
    .bind((host, port))
    .map(|server| {
        info!("Server is running successfully");
        server
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server failed to run: {}", e);
        e
    })?;

    info!("Server shutdown successfully");

    Ok(())
}
