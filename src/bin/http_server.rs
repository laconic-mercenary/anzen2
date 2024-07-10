
use actix::{Addr, Actor};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anzen2::{stream_server::StreamServer, stream_session::StreamSession, time};

use std::{env::set_var, fs};

const ENV_ALLOWED_CIDR: &str = "ALLOWED_CIDR";
const ANY_CIDR: &str = "*";
const CONTENT_TYPE_JS: &str = "application/javascript; charset=utf-8";
const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
const PAYLOAD_SIZE : usize = 10 * 1024 * 1024; // payload buffer size

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    set_var("RUST_LOG", "debug");
    log::info!("starting server...");

    let stream_server = StreamServer::new().start();
    let workers_ct = num_cpus::get() * 2;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(stream_server.clone()))
            .app_data(web::PayloadConfig::new(PAYLOAD_SIZE))
            .service(web::resource("/device").route(web::get().to(get_device_page)))
            .service(web::resource("/monitor").route(web::get().to(get_monitor_page)))
            .service(web::resource("/js/monitor.js").route(web::get().to(get_monitor_js)))
            .service(web::resource("/js/device.js").route(web::get().to(get_device_js)))
            .route("/ws/", web::get().to(start_monitor_websocket))
    })
    .bind("0.0.0.0:8080")?
    .workers(workers_ct)
    .run()
    .await
}


async fn get_monitor_js() -> HttpResponse {
    let html = fs::read_to_string("static/js/monitor.js").unwrap();
    HttpResponse::Ok().content_type(CONTENT_TYPE_JS).body(html)
}

async fn get_monitor_page() -> HttpResponse {
    let html = fs::read_to_string("static/monitor.html").unwrap();
    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_HTML)
        .body(html)
}

async fn get_device_js() -> HttpResponse {
    let html = fs::read_to_string("static/js/device.js").unwrap();
    HttpResponse::Ok().content_type(CONTENT_TYPE_JS).body(html)
}

async fn get_device_page() -> HttpResponse {
    let html = fs::read_to_string("static/device.html").unwrap();
    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_HTML)
        .body(html)
}

async fn start_monitor_websocket(
    req: HttpRequest, 
    stream: web::Payload,
    srv: web::Data<Addr<StreamServer>>
) -> Result<HttpResponse, Error> {
    log::trace!("start_monitor_websocket");
    let conn_info = req.connection_info();
    let remote_addr = conn_info.peer_addr().unwrap();
    if !is_ip_in_cidr(remote_addr.parse().unwrap()) {
        log::warn!("connection from {} is not allowed", remote_addr);
        return Ok(HttpResponse::Forbidden().body("Forbidden"));
    }
    
    let stream_session = StreamSession::new(srv.get_ref().clone());
    log::info!(
        "starting websocket connection: {} {} from {}",
        req.method(),
        req.uri().path(),
        remote_addr
    );
    ws::start(
        stream_session, 
        &req, 
        stream
    )
}

fn is_ip_in_cidr(ip: std::net::IpAddr) -> bool {
    let env_check = std::env::var(ENV_ALLOWED_CIDR);
    let cidr_str = env_check.unwrap_or(ANY_CIDR.to_string());
    if cidr_str == ANY_CIDR {
        true
    } else {
        let cidr: ipnetwork::IpNetwork = cidr_str.parse().expect("Failed to parse CIDR");
        cidr.contains(ip)
    }
}
