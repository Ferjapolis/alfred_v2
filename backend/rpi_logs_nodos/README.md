### Sistema Domótico Alfred v2
# API de servicio de recolección de datos
Esta API desarrollada con `Actix-web` está diseñada para recopilar, almacenar y consultar datos operativos de nodos basados en Arduino que controlan sensores PIR y relés. La implementación se centra en un sistema de automatización que funciona en una Raspberry Pi Zero 2, con soporte para la escritura y consulta de datos en una base de datos `InfluxDB`.

## Objetivos
El objetivo de este sistema es crear un flujo eficiente de información entre los sensores de los nodos Arduino y una base de datos robusta como InfluxDB. Cada vez que un sensor PIR detecta movimiento en un rincón de tu hogar, o un nodo registra cambios en la temperatura, ese dato no se queda en el aire. En su lugar, es enviado a la Raspberry Pi, que se encarga de organizar, procesar y guardar esa información en su base de datos con precisión quirúrgica. Esto convierte la colección de datos en una cronología perfectamente sincronizada y consultable.

1. Recolectar datos de sensores PIR y de ambiente enviados por los Arduinos.
2. Guardar estados de relés activados por los nodos.
3. Almacenar metadatos de nodos Arduino, como ubicación y descripción.
4. Proveer acceso a los datos históricos almacenados en InfluxDB, con filtros por tiempo.

### Diagrama de comunicaciones
#### Backend a Frontend
![conexion](/img/server-frontend.png)

#### Backend a Nodos
![conexion](/img/server-nodos.png)

## Rutas de la API

### 1. Sensor Data
- **Ruta**: `POST` /sensors
- **Propósito**: Guardar datos de sensores ambientales y PIR enviados por los nodos Arduino.
- **Modelo de datos**:
    ```json
    {
    "nodo": "nodo_1",
    "temperature": 25.6,
    "humidity": 60.5,
    "pir0": true,
    "pir1": false,
    "pir2": true
    }
    ```
- **Procesamiento**:
    - Los datos se almacenan en la tabla sensors de InfluxDB.
    - Se añaden etiquetas (tags) y campos (fields) relevantes al punto de datos.

### 2. Relay Data
- **Ruta**: `POST` /reles
- **Propósito**: Registrar el estado de un relé controlado por un nodo Arduino.
- **Modelo de datos**:
    ```json
    {
    "nodo": "nodo_1",
    "relay": 1,
    "state": true
    }
    ```
- **Procesamiento**:
    - Los datos se almacenan en la tabla reles de InfluxDB.
    - Cada entrada incluye el identificador del nodo, el número del relé y su estado (encendido/apagado).

### 3. PIR Data
- **Ruta**: `POST` /pir
- **Propósito**: Guardar el estado de un sensor PIR individual en un nodo.
- **Modelo de datos**:
    ```json
    {
    "nodo": "nodo_1",
    "pir": 0,
    "state": true
    }
    ```
- **Procesamiento**:
    - Los datos se almacenan en la tabla pir de InfluxDB.
    - Incluyen información del nodo y el estado del sensor PIR específico.



### 4. Consultar Datos por Nodo
- **Ruta**: `GET` /{tipo}/{nodo}/{period}
    - Ejemplo: `GET` /sensors/nodo_1/day
- **Propósito**: Obtener datos históricos de sensores, relés o PIR, filtrados por nodo y período de tiempo.
- **Parámetros**:
    - **tipo**: Puede ser `sensors`, `reles` o `pir`.
    - **nodo**: Identificador del nodo que se desea consultar.
    - **period**: Rango temporal (`day`, `week`, `month`).
- **Procesamiento**:
    - Se consulta la base de datos InfluxDB usando un rango temporal:
        - Último día: `time >= now() - 1d`
        - Última semana: `time >= now() - 7d`
        - Último mes: `time >= now() - 30d`
- **Respuesta**: Datos en formato JSON. Ejemplo para sensores:
    ```json
    [
        {
            "nodo": "nodo_1",
            "temperature": 25.6,
            "humidity": 60.5,
            "pir0": true,
            "pir1": false,
            "pir2": true,
            "timestamp": "2023-12-01T12:34:56Z"
        }
    ]
    ```

### 5. Consultar Metadatos de Nodos
- **Ruta**: `GET` /nodos
- **Propósito**: Obtener información sobre los nodos registrados, como ubicación y descripción.
- **Respuesta**:
    ```json
    [
        {
            "nodo": "nodo_1",
            "description": "Sensor en el pasillo",
            "location": "Casa - Primer piso"
        },
        {
            "nodo": "nodo_2",
            "description": "Relés en la sala",
            "location": "Casa - Sala"
        }
    ]
    ```

## Estructura de Datos
### 1. Sensor Data
| Campo | Tipo | Descripción | 
|---|---|---|
| `nodo` | String | Identificador del nodo. | 
| `temperature` | f32 | Temperatura medida (°C). | 
| `humidity` | f32 | Humedad medida (%). | 
| `pir0` | bool | Estado del PIR 0. | 
| `pir1` | bool | Estado del PIR 1. | 
| `pir2` | bool | Estado del PIR 2. | 
| `timestamp` | DateTime<Utc> | Marca de tiempo. | 

### 2. Relay Data
| Campo | Tipo | Descripción |
|---|---|---|
| `nodo` | String | Identificador del nodo. |
| `relay` | u8 | Número del relé. |
| `state` | bool | Estado del relé (on/off). |
| `timestamp` | DateTime<Utc> | Marca de tiempo. |

### 3. PIR Data
| Campo | Tipo | Descripción | 
|---|---|---|
| `nodo` | String | Identificador del nodo. | 
| `pir` | u8 | Número del PIR. | 
| `state` | bool | Estado del PIR (detecto/no detecto). | 
| `timestamp` | DateTime<Utc> | Marca de tiempo. | 

### 4. Node Metadata
| Campo | Tipo | Descripción | 
|---|---|---|
| `nodo` | String | Identificador del nodo. | 
| `description` | String | Descripción del nodo. | 
| `location` | String | Ubicación física del nodo. | 

## Librerías Utilizadas
En este proyecto, se emplean diversas librerías para manejar los componentes clave del sistema:

```toml
[package]
name = "api_v1"
version = "1.0.0"
edition = "2021"
authors = ["Ferjapolis"]
description = "Api de manejo de recursos para sistema domotico"

[dependencies]
actix-web = "4.4"
serde = { version = "1.0", features = ["derive"] }
influxdb = { version = "0.7", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
tokio = { version = "1.0", features = ["full"] }
```

- `actix-web` : Es un framework rápido y asíncrono para construir servidores web en Rust. Maneja las rutas, solicitudes HTTP y respuestas.
- `serde` y `serde_json`: Herramientas poderosas para serializar y deserializar datos en formato JSON.
- `influxdb`: Cliente para interactuar con la base de datos InfluxDB, optimizada para datos de series temporales.
- `chrono`: Librería para manejar y formatear marcas de tiempo (timestamps) en el proyecto.
- `futures`: Proporciona soporte para programación asíncrona, necesaria para manejar las solicitudes HTTP y operaciones con la base de datos sin bloquear el servidor.


## Mejoras Futuras
- Implementar autenticación para proteger las rutas.
- Añadir soporte para más tipos de sensores y actuadores.
- Optimizar las consultas a InfluxDB con índices y funciones de agregación.
- Exponer métricas para monitoreo del rendimiento de la API.