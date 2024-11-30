use actix_web::{web, App, HttpServer, Responder, Result};
use local_ip_address::local_ip;
use serde::Serialize;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind};

#[derive(Debug, Serialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
}

#[derive(Debug, Serialize)]
struct SystemStatus {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    ip_address: String,
}

#[derive(thiserror::Error, Debug)]
pub enum MonitorError {
    #[error("Failed to serialize data: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("System info error: {0}")]
    SystemError(String),
}

async fn status() -> Result<impl Responder> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::new()),
    );
    
    // Refrescar la información del sistema
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let status = SystemStatus {
        cpu_usage: sys.global_cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        ip_address: get_local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "127.0.0.1".to_string()),
    };

    Ok(web::Json(status))
}

async fn process() -> Result<impl Responder, actix_web::Error> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut process_info = Vec::new();
    for (pid, process) in sys.processes() {
        process_info.push(ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),  // Corregido aquí
            cpu_usage: process.cpu_usage(),
        });
    }

    // Ordenar procesos por uso de CPU (descendente)
    process_info.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    Ok(web::Json(process_info))
}

fn get_local_ip() -> Option<std::net::IpAddr> {
    local_ip().ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api")
                    .route("/status", web::get().to(status))
                    .route("/process", web::get().to(process)),
            )
    })
    .bind((host, port))?
    .run()
    .await
}