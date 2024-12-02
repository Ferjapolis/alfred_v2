### Sistema Domótico Alfred v2
# API para Monitoreo de Sistema y Hardware en Raspberry Pi Zero 2

## Introducción
Este proyecto implementa una API REST utilizando Rust, el framework Actix-Web, y diversas bibliotecas para monitorear el estado del sistema y hardware, así como información relacionada con contenedores e imágenes Docker. Está diseñado para ejecutarse en una Raspberry Pi Zero 2, pero puede adaptarse para otras plataformas compatibles con Rust.

La API proporciona endpoints que devuelven información como el uso de CPU, memoria, procesos en ejecución, contenedores Docker activos e imágenes almacenadas en el sistema.

## Objetivos
La API está diseñada para monitorear y compartir información detallada sobre el sistema operativo y su rendimiento. Es ideal para entornos de monitoreo, donde es crucial obtener datos en tiempo real sobre el estado del sistema y las operaciones de contenedores Docker. La API es ligera y está optimizada para ejecutarse en dispositivos de bajo consumo como la Raspberry Pi Zero 2, proporcionando una herramienta versátil para supervisar sistemas operativos y contenedores.
- **Estado del Sistema**: Devuelve métricas como el uso de CPU, memoria total y usada, y la dirección IP del sistema.
- **Procesos Activos**: Lista los procesos activos, incluyendo su ID, nombre y uso de CPU.
- **Contenedores Docker**: Lista los contenedores Docker, mostrando su ID, nombre, estado, imagen asociada, puertos expuestos, tamaño y tiempo desde su creación.
- **Imágenes Docker**: Lista las imágenes Docker almacenadas en el sistema, incluyendo el repositorio, etiquetas, tamaño y tiempo desde su creación.

### Diagrama de comunicaciones
#### Backend a Frontend
![conexion](/img/api_pri_monitor.png)

## Rutas de la API
### 1. Estado del Sistema
- **Ruta**: `GET` /api/status
- **Propósito**: Proporcionar información sobre el estado del sistema, incluyendo el uso de CPU, memoria y la dirección IP del dispositivo.
- **Modelo de datos**:
    ```json
    {
        "cpu_usage": 15.2,
        "total_memory": 1024000,
        "used_memory": 512000,
        "ip_address": "192.168.1.100"
    }
    ```
- **Procesamiento**:
    - Se recopilan los datos del sistema utilizando la biblioteca sysinfo.
    - La API calcula el uso global de la CPU, la memoria total y utilizada.
    - La dirección IP del sistema se obtiene mediante local_ip_address.
    - Los datos se devuelven en formato JSON.

### 2. Procesos Activos
- **Ruta**: `GET` /api/process
- **Propósito**: Listar los procesos en ejecución en el sistema, incluyendo su ID, nombre y porcentaje de uso de CPU.
- **Modelo de datos**:
    ```json
    [
        {
            "pid": 1234,
            "name": "bash",
            "cpu_usage": 2.3
        },
        {
            "pid": 5678,
            "name": "nginx",
            "cpu_usage": 5.7
        }
    ]
    ```
- **Procesamiento**:
    - La biblioteca sysinfo obtiene la lista de procesos activos.
    - Para cada proceso, se extraen el ID (PID), nombre y uso de CPU.
    - Los datos se devuelven como un arreglo de objetos JSON.

### 3. Contenedores Docker
- **Ruta**: `GET` /api/containers
- **Propósito**: Listar los contenedores Docker en el sistema, incluyendo su estado, imagen asociada, tamaño, y puertos expuestos.
- **Modelo de datos**:
    ```json
    {
        "Containers": [
            {
            "id": "abcd1234",
            "name": "my_container",
            "status": "Up 10 minutes",
            "state": "running",
            "image": "nginx:latest",
            "created": "2 hours ago",
            "size": 1048576,
            "ports": ["192.168.1.100:80->8080"]
            }
        ]
    }
    ```
- **Procesamiento**:
    - Se conecta al demonio de Docker usando la biblioteca bollard.
    - Se listan todos los contenedores, tanto activos como inactivos.
    - Para cada contenedor, se extraen detalles como su ID, nombre, estado, imagen asociada, puertos expuestos, tamaño y fecha de creación.
    - Los datos se formatean y se devuelven en formato JSON.

### 4. Imágenes Docker
- **Ruta**: `GET` /api/images
- **Propósito**: Listar las imágenes Docker almacenadas en el sistema, incluyendo el repositorio, etiquetas, tamaño y tiempo desde su creación.
- **Respuesta**: Datos en formato JSON. Ejemplo para sensores:
    ```json
    {
        "Images": [
            {
                "repository": "nginx",
                "tag": "latest",
                "id": "sha256:abcd1234",
                "created": "3 days ago",
                "size": "23.5MB"
            }
        ]
    }
    ```
- **Procesamiento**:
    - Se conecta al demonio de Docker usando la biblioteca bollard.
    - Se listan todas las imágenes almacenadas en el sistema.
    - Para cada imagen, se obtienen detalles como el repositorio, etiquetas, ID, tamaño y fecha de creación.
    - Los datos se formatean y se devuelven en formato JSON.

### Errores en Docker
Si se producen errores al conectar al demonio de Docker o al listar contenedores/imágenes, las rutas `/api/containers` y `/api/images` devuelven una respuesta de error con el siguiente modelo de datos:
Modelo de datos (Error):
- **Respuesta**:
    ```json
    {
        "error": "Failed to connect to Docker daemon",
        "details": "Error: No such file or directory"
    }
    ```
- **Procesamiento**:
    - El error se captura y se envía al cliente en un formato legible, incluyendo un mensaje genérico y detalles específicos del fallo.

## Estructura de Datos
### 1. Estado del Sistema
| Campo | Tipo | Descripción | 
|---|---|---|
| `cpu_usage` | f32 | Uso global de la CPU en porcentaje. |
| `total_memory` | u64 | Memoria total del sistema en kilobytes. |
| `used_memory` | u64 | Memoria utilizada del sistema en kilobytes. |
| `ip_address` | String | Dirección IP local del dispositivo. |

### 2. Procesos Activos
| Campo | Tipo | Descripción |
|---|---|---|
| `pid` | u32 | Identificador único del proceso (PID). |
| `name` | String | Nombre del proceso. |
| `cpu_usage` | f32 | Porcentaje de uso de CPU del proceso específico. |

### 3. Contenedores Docker
| Campo | Tipo | Descripción | 
|---|---|---|
| `id` | String | Identificador único del contenedor. |
| `name` | String | Nombre(s) del contenedor |
| `status` | String | Estado del contenedor |
| `state` | String | Estado técnico del contenedor |
| `image` | String | Nombre de la imagen utilizada por el contenedor. |
| `created` | String | Tiempo transcurrido desde la creación del contenedor|
| `size` | u64 | Tamaño del contenedor en bytes. |
| `ports` | Vec | Lista de puertos mapeados y expuestos por el contenedor. |

### 4. Imágenes Docker
| Campo | Tipo | Descripción | 
|---|---|---|
| `repository` | String | Nombre del repositorio de la imagen |
| `tag` | String | Etiqueta de la imagen |
| `id` | String | Identificador único de la imagen. |
| `created` | String | Tiempo transcurrido desde la creación de la imagen |
| `size` | String | Tamaño de la imagen en formato legible |

## Librerías Utilizadas
En este proyecto, se emplean diversas librerías para manejar los componentes clave del sistema:

```toml
[package]
name = "rpi_monitor"
version = "0.1.0"
edition = "2021"
authors = ["Ferjapolis"]
description = "API de datos de sistemas"

[[bin]]
name = "rpi_monitor"
path = "src/main.rs"

[dependencies]
actix-web = "4.4"
sysinfo = "0.32.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
local-ip-address = "0.6.3"
thiserror = "2.0.3"
log = "0.4"
env_logger = "0.11.5"
reqwest = "0.12.9"
bollard = "0.18.1"
futures-util = "0.3"
chrono = "0.4"
```

- `actix-web` : Es un framework rápido y asíncrono para construir servidores web en Rust. Maneja las rutas, solicitudes HTTP y respuestas.
- `Sysinfo`: Extrae información del sistema y procesos activos.
- `Bollard`: Interactúa con la API de Docker para manejar contenedores e imágenes.
- `serde` y `serde_json`: Herramientas poderosas para serializar y deserializar datos en formato JSON.
- `influxdb`: Cliente para interactuar con la base de datos InfluxDB, optimizada para datos de series temporales.
- `chrono`: Librería para manejar y formatear marcas de tiempo (timestamps) en el proyecto.
- `Local IP Address`: Obtiene la dirección IP local del dispositivo.
- `futures`: Proporciona soporte para programación asíncrona, necesaria para manejar las solicitudes HTTP y 

## Mejoras Futuras
- Implementar autenticación para proteger las rutas.
- Añadir soporte para más tipos de sensores y actuadores.
- Optimizar las consultas a InfluxDB con índices y funciones de agregación.
- Exponer métricas para monitoreo del rendimiento de la API.