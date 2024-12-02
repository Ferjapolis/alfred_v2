use actix_web::{web, Responder, Result, HttpServer};
use bollard::container::ListContainersOptions;
use bollard::image::ListImagesOptions;
use bollard::Docker;
use local_ip_address::local_ip;
use serde::Serialize;
use sysinfo::{System, Process};
use chrono::{DateTime, Utc};

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

#[derive(Debug, Serialize)]
struct ContainerInfo {
    id: String,
    name: String,
    status: String,
    state: String,
    image: String,
    created: String,
    size: u64,
    ports: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ImageInfo {
    repository: String,
    tag: String,
    id: String,
    created: String,
    size: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    details: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum DockerResponse {
    Containers(Vec<ContainerInfo>),
    Images(Vec<ImageInfo>),
    Error(ErrorResponse)
}

async fn connect_docker() -> Result<Docker, bollard::errors::Error> {
    if let Ok(docker) = Docker::connect_with_unix_defaults() {
        return Ok(docker);
    }
    if let Ok(docker) = Docker::connect_with_local_defaults() {
        return Ok(docker);
    }
    Docker::connect_with_socket_defaults()
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1}KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1}MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2}GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_timestamp(timestamp: i64) -> String {
    if let Some(datetime) = DateTime::from_timestamp(timestamp, 0) {
        let duration = Utc::now().signed_duration_since(datetime);

        if duration.num_days() > 30 {
            format!("{} months ago", duration.num_days() / 30)
        } else if duration.num_days() > 0 {
            format!("{} days ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{} hours ago", duration.num_hours())
        } else {
            format!("{} minutes ago", duration.num_minutes())
        }
    } else {
        "unknown time".to_string()
    }
}

async fn status() -> Result<impl Responder> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let ip = local_ip()
        .unwrap_or_else(|_| "Unknown".parse().unwrap());

    let status = SystemStatus {
        cpu_usage: sys.global_cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        ip_address: ip.to_string(),
    };

    Ok(web::Json(status))
}

async fn process() -> Result<impl Responder> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let processes: Vec<ProcessInfo> = sys
        .processes()
        .values()
        .map(|process| ProcessInfo {
            pid: process.pid().as_u32(),
            name: process.name().to_str().unwrap_or("").to_string(),
            cpu_usage: process.cpu_usage(),
        })
        .collect();

    Ok(web::Json(processes))
}

async fn containers() -> Result<impl Responder, actix_web::Error> {
    let docker = match connect_docker().await {
        Ok(docker) => docker,
        Err(e) => {
            return Ok(web::Json(DockerResponse::Error(ErrorResponse {
                error: "Failed to connect to Docker daemon".to_string(),
                details: Some(format!("Error: {}", e)),
            })));
        }
    };

    let options = Some(ListContainersOptions::<String> {
        all: true,
        size: true,
        ..Default::default()
    });

    match docker.list_containers(options).await {
        Ok(containers) => {
            let container_info: Vec<ContainerInfo> = containers
                .iter()
                .map(|container| {
                    let ports = container.ports.as_ref()
                        .map(|ports| {
                            ports.iter()
                                .filter_map(|port| {
                                    let private_port = port.private_port.to_string();
                                    let public_port = port.public_port.map(|p| p.to_string());
                                    let ip = port.ip.clone();

                                    match (public_port, ip) {
                                        (Some(pub_port), Some(ip)) =>
                                            Some(format!("{}:{}->{}", ip, pub_port, private_port)),
                                        (Some(pub_port), None) =>
                                            Some(format!("{}:{}", pub_port, private_port)),
                                        _ => Some(private_port)
                                    }
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    ContainerInfo {
                        id: container.id.clone().unwrap_or_default(),
                        name: container.names.clone()
                            .unwrap_or_default()
                            .iter()
                            .map(|name| name.trim_start_matches('/').to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                        status: container.status.clone().unwrap_or_default(),
                        state: container.state.clone().unwrap_or_default(),
                        image: container.image.clone().unwrap_or_default(),
                        created: format_timestamp(container.created.unwrap_or(0)),
                        size: container.size_rw.unwrap_or(0) as u64,
                        ports,
                    }
                })
                .collect();
            Ok(web::Json(DockerResponse::Containers(container_info)))
        }
        Err(e) => {
            Ok(web::Json(DockerResponse::Error(ErrorResponse {
                error: "Failed to list containers".to_string(),
                details: Some(e.to_string()),
            })))
        }
    }
}

async fn images() -> Result<impl Responder, actix_web::Error> {
    let docker = match connect_docker().await {
        Ok(docker) => docker,
        Err(e) => {
            return Ok(web::Json(DockerResponse::Error(ErrorResponse {
                error: "Failed to connect to Docker daemon".to_string(),
                details: Some(format!("Error: {}", e)),
            })));
        }
    };

    let options = Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    });

    match docker.list_images(options).await {
        Ok(images) => {
            let image_info: Vec<ImageInfo> = images
                .iter()
                .map(|image| {
                    let repo_tags = image.repo_tags.get(0)
                        .cloned()
                        .unwrap_or_else(|| "none:none".to_string());
                    let mut parts = repo_tags.splitn(2, ':');
                    let repository = parts.next().unwrap_or("").to_string();
                    let tag = parts.next().unwrap_or("").to_string();

                    ImageInfo {
                        repository,
                        tag,
                        id: image.id.clone(),
                        created: format_timestamp(image.created),
                        size: format_size(image.size as u64),
                    }
                })
                .collect();
            Ok(web::Json(DockerResponse::Images(image_info)))
        }
        Err(e) => {
            Ok(web::Json(DockerResponse::Error(ErrorResponse {
                error: "Failed to list images".to_string(),
                details: Some(e.to_string()),
            })))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api")
                    .route("/status", web::get().to(status))
                    .route("/process", web::get().to(process))
                    .route("/containers", web::get().to(containers))
                    .route("/images", web::get().to(images))
            )
    })
    .bind((host, port))?
    .run()
    .await
}
