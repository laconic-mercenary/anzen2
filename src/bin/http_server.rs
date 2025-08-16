
use actix::{Addr, Actor};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anzen2::{config, stream_server::StreamServer, stream_session::VideoSession};

use std::fs;

const LOCALHOST_BIND_ADDR: &str = "127.0.0.1:8080";
const EXPOSED_BIND_ADDR: &str = "0.0.0.0:8080";
const WORKERS_MULTIPLIER: usize = 2;
const ENV_ALLOWED_CIDR: &str = "ALLOWED_CIDR";
const ANY_CIDR: &str = "*";
const CONTENT_TYPE_JS: &str = "application/javascript; charset=utf-8";
const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
const PAYLOAD_SIZE : usize = 1024 * 1024; // payload buffer size
const PROXY_FWD_HEADER: &str = "X-Forwarded-For";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("starting server...");

    let stream_server = StreamServer::new().start();
    let workers_ct = num_cpus::get() * WORKERS_MULTIPLIER;
    let mut bind_addr = LOCALHOST_BIND_ADDR.to_string();
    if !config::bind_localhost_addr() {
        bind_addr = EXPOSED_BIND_ADDR.to_string();
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(stream_server.clone()))
            .service(web::resource("/device").route(web::get().to(get_device_page)))
            .service(web::resource("/monitor").route(web::get().to(get_monitor_page)))
            .service(web::resource("/mtx-monitor").route(web::get().to(get_mtx_monitor_page)))
            .service(web::resource("/js/monitor.js").route(web::get().to(get_monitor_js)))
            .service(web::resource("/js/device.js").route(web::get().to(get_device_js)))
            .service(web::resource("/js/mtx-monitor.js").route(web::get().to(get_mtx_monitor_js)))
            .route("/ws/", web::get().to(start_monitor_websocket))
    })
    .bind(bind_addr)?
    .workers(workers_ct)
    .run()
    .await
}

async fn get_monitor_js() -> HttpResponse {
    log::trace!("get_monitor_js");
    let html = fs::read_to_string("static/js/monitor.js").unwrap();
    HttpResponse::Ok().content_type(CONTENT_TYPE_JS).body(html)
}

async fn get_monitor_page() -> HttpResponse {
    log::trace!("get_monitor_page");
    let html = fs::read_to_string("static/monitor.html").unwrap();
    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_HTML)
        .body(html)
}

async fn get_device_js() -> HttpResponse {
    log::trace!("get_device_js");
    let html = fs::read_to_string("static/js/device.js").unwrap();
    HttpResponse::Ok().content_type(CONTENT_TYPE_JS).body(html)
}

async fn get_device_page() -> HttpResponse {
    log::trace!("get_device_page");
    let html = fs::read_to_string("static/device.html").unwrap();
    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_HTML)
        .body(html)
}

async fn get_mtx_monitor_page() -> HttpResponse {
    log::trace!("get_mtx_monitor_page");
    let html = fs::read_to_string("static/mtx-monitor.html").unwrap();
    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_HTML)
        .body(html)
}

async fn get_mtx_monitor_js() -> HttpResponse {
    log::trace!("get_mtx_monitor_js");
    match fs::read_to_string("static/js/mtx-monitor.js") {
        Ok(js) => HttpResponse::Ok().content_type(CONTENT_TYPE_JS).body(js),
        Err(e) => {
            log::error!("Failed to read mtx-monitor.js: {}", e);
            HttpResponse::NotFound().body("mtx-monitor.js not found")
        }
    }
}
 
async fn start_monitor_websocket(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<StreamServer>>,
) -> Result<HttpResponse, Error> {
    log::trace!("start_monitor_websocket");
    let conn_info = req.connection_info();
    let headers = req.headers();
    let remote_addr = headers
        .get(PROXY_FWD_HEADER)
        .and_then(|x| x.to_str().ok())
        .unwrap_or_else(|| conn_info.peer_addr().unwrap());

    if !is_ip_in_cidr(remote_addr.parse().unwrap()) {
        log::warn!("connection from {} is not allowed", remote_addr);
        return Ok(HttpResponse::Forbidden().body("forbidden"));
    }

    let stream_session = VideoSession::new(srv.get_ref().clone());
    log::info!(
        "starting websocket connection: {} {} from {}",
        req.method(),
        req.uri().path(),
        remote_addr
    );
    //ws::start(stream_session, &req, stream)
    ws::WsResponseBuilder::new(stream_session, &req, stream)
        .frame_size(PAYLOAD_SIZE)
        .start()
}

fn is_ip_in_cidr(ip: std::net::IpAddr) -> bool {
    let env_check = std::env::var(ENV_ALLOWED_CIDR);
    let cidr_str = env_check.unwrap_or(ANY_CIDR.to_string());
    if cidr_str == ANY_CIDR {
        true
    } else {
        let cidr: ipnetwork::IpNetwork = cidr_str.parse().unwrap();
        cidr.contains(ip)
    }
}
