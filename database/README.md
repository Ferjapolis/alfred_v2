### Sistema Domótico Alfred v2
# Database

InfluxDB fue seleccionada como la base de datos para este proyecto debido a su diseño específico para manejar datos de series temporales, que son fundamentales en aplicaciones IoT y domótica. Aquí se detallan las razones clave detrás de esta decisión:

## InfluxDB en Raspberry Pi Zero 2 W
Esta documentación cubre los pasos para instalar, configurar y estructurar las tablas (series) en InfluxDB. InfluxDB es ideal para proyectos de IoT y domótica debido a su diseño optimizado para manejar datos de series temporales.

## Instalación de InfluxDB
### 1. Prerrequisitos
Antes de instalar InfluxDB, asegúrate de que tu Raspberry Pi Zero 2 W cumpla con los siguientes requisitos:
  - Sistema operativo: Raspberry Pi OS (32-bit) o similar basado en Debian.
  - Conexión a Internet.
  - Dependencias básicas: curl y wget instalados.

### 2. Instalación
Instalar InfluxDB usando el repositorio oficial
- Actualizar el sistema:
  ```bash
  sudo apt update && sudo apt upgrade -y
  ```

- Agregar el repositorio de InfluxDB:
  ```bash
  wget -qO- https://repos.influxdata.com/influxdb.key | sudo apt-key add -
  echo "deb https://repos.influxdata.com/debian buster stable" | sudo tee /etc/apt/sources.list.d/influxdb.list
  ```

- Instalar InfluxDB:
  ```bash
  sudo apt update
  sudo apt install -y influxdb
  ```

- Iniciar y habilitar el servicio:
  ```bash
  sudo systemctl start influxdb
  sudo systemctl enable influxdb
  ```

- Verificar el estado:
  ```bash
  sudo systemctl status influxdb
  ```

#### 3. Configuración Inicial
Acceso al CLI de InfluxDB
- Ejecuta el cliente de InfluxDB para interactuar con la base de datos:
  ```bash
  influx
  ```

- Configura una base de datos inicial:
  ```sql
  CREATE DATABASE domotica;
  ```

- Lista las bases de datos para confirmar:
  ```sql
  SHOW DATABASES;
  ```

### Configurar Retención de Datos
La retención de datos define cuánto tiempo se mantendrán los registros antes de ser eliminados:
```sql
CREATE RETENTION POLICY "30dias" ON domotica DURATION 30d REPLICATION 1 DEFAULT;
```

## Estructura de las Tablas (Series)
En InfluxDB, los datos se almacenan en series que son organizadas por:
- **Mediciones**: Las entidades principales (equivalente a "tabla" en SQL).
- **Etiquetas (tags)**: Metadatos indexados para búsquedas rápidas.
- **Campos (fields)**: Los valores principales a almacenar.
- **Tiempos (timestamps)**: Marca de tiempo para cada registro.

#### 1. Diseño de la Base de Datos
La base de datos `domotica` estará compuesta por las siguientes mediciones principales:

#### 1.1 Medición: sensors
- **Descripción**: Registra las lecturas de sensores de temperatura y humedad.
- **Estructura**:
  - **Tags**:
    - `nodo` (string): Identificador del nodo.
    - `sensor_id` (string): ID del sensor.
  - **Fields**:
    - `temperature` (float): Temperatura en grados Celsius.
    - `humidity` (float): Humedad relativa (%).
- Ejemplo de Registro:
  ```
  sensors,nodo=nodo1,sensor_id=DHT22 temperature=22.5,humidity=60.2 1638206400000000000
  ```

#### 1.2 Medición: reles
- **Descripción**: Almacena los cambios de estado en los relés.
- **Estructura**:
  - **Tags**:
    - `nodo` (string): Identificador del nodo.
    - `relay_id` (integer): ID del relé.
  - **Fields**:
    - `state` (boolean): Estado del relé (true: activado, false: desactivado).
- Ejemplo de Registro:
  ```c#
  reles,nodo=nodo1,relay_id=1 state=true 1638206400000000000
  ```

#### 1.3 Medición: pir
- **Descripción**: Registra activaciones de sensores PIR.
- **Estructura**:
  - **Tags**:
    - `nodo` (string): Identificador del nodo.
    - `pir_id` (integer): ID del sensor PIR.
  - **Fields**:
    - `state` (boolean): Estado del sensor (true: activado, false: desactivado).
- Ejemplo de Registro:
  ```c#
  pir,nodo=nodo1,pir_id=0 state=true 1638206400000000000
  ```

### Configuración del Backend
#### Biblioteca de Rust para InfluxDB
Para integrar InfluxDB con el backend en Rust, usa la biblioteca `influxdb`:

1. Agrega la dependencia al archivo `Cargo.toml`:
```toml
[dependencies]
influxdb = "0.4"
influxdb-derive = "0.4"
tokio = { version = "1", features = ["full"] }
```

2. Ejemplo de código para escribir datos en InfluxDB:
```rust
use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable)]
struct SensorData {
    #[influxdb(tag)] nodo: String,
    #[influxdb(tag)] sensor_id: String,
    temperature: f64,
    humidity: f64,
}

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:8086", "domotica");
    let data = SensorData {
        nodo: "nodo1".to_string(),
        sensor_id: "DHT22".to_string(),
        temperature: 22.5,
        humidity: 60.2,
    };

    let query = data.into_query("sensors").build().unwrap();
    client.query(&query).await.unwrap();
```

## Por qué Elegir InfluxDB para el Proyecto
### 1. Optimización para Datos Temporales
InfluxDB está diseñada específicamente para datos que incluyen marcas de tiempo, como las lecturas de sensores o registros de eventos. En este proyecto, cada dato (temperatura, humedad, estado de relés, activaciones PIR) está relacionado con un momento exacto, lo que la hace ideal para:
- Captura de datos continuos: Permite almacenar datos generados constantemente por nodos IoT.
- Consultas eficientes: Ofrece operaciones avanzadas para trabajar con intervalos de tiempo (e.g., últimos 10 minutos, día anterior).

### 2. Ligereza y Eficiencia en Hardware Limitado
El proyecto está diseñado para ejecutarse en una Raspberry Pi Zero 2 W, que tiene recursos de hardware limitados:
- InfluxDB es ligera: Comparada con otras bases de datos como MongoDB o PostgreSQL, InfluxDB tiene un menor consumo de CPU y memoria para operaciones de lectura/escritura de series temporales.
- Eficiencia energética: Ideal para sistemas pequeños y de bajo consumo como la Raspberry Pi Zero.

### 3. Funcionalidades Clave para IoT
InfluxDB proporciona herramientas esenciales para gestionar datos en aplicaciones de IoT:
- Políticas de retención de datos: Permite descartar automáticamente datos antiguos que ya no sean relevantes (e.g., mantener solo datos de los últimos 30 días).
- Compresión de datos: Almacena grandes volúmenes de datos en un espacio reducido.
- Tags y Fields: Diferencia entre datos indexados para búsquedas rápidas (tags) y datos medidos (fields), mejorando la eficiencia de consultas en sistemas con múltiples sensores.

### 4. Consultas Avanzadas y Analíticas
InfluxDB permite realizar análisis directamente sobre los datos almacenados sin necesidad de herramientas externas:
- Operaciones matemáticas: Cálculos como promedio, suma, y máximos/mínimos son nativos.
- Agregaciones temporales: Es fácil calcular estadísticas en intervalos definidos (e.g., cada hora, por día).
- Flexibilidad en consultas: A través de su lenguaje de consulta (InfluxQL) o Flux, ofrece poderosas herramientas analíticas para generar reportes o insights en tiempo real.

### 5. Compatibilidad y Ecosistema
InfluxDB es altamente compatible con herramientas y servicios de IoT:
- Integración con Rust: Mediante bibliotecas como influxdb para enviar y consultar datos.
- Soporte para Grafana: Puede visualizar datos almacenados en InfluxDB fácilmente a través de paneles interactivos, ideales para monitoreo en tiempo real.
- APIs HTTP: Facilita la comunicación entre los nodos IoT y la base de datos mediante endpoints REST.

### 6. Escalabilidad
Aunque el proyecto comienza en una Raspberry Pi Zero 2 W, InfluxDB está diseñada para crecer con las necesidades del sistema:
- Manejo de grandes volúmenes de datos: Puede gestionar millones de registros por segundo.
- Fácil migración: Si en el futuro se requiere más capacidad, se puede trasladar a una infraestructura más robusta (e.g., servidores en la nube).

### Comparación con Otras Bases de Datos
| Base de Datos | Ventaja | Desventaja |
|---|---|---|
| **InfluxDB** | Optimizada para series temporales, ligera, eficiente en hardware limitado. | Enfocada exclusivamente en datos temporales, no ideal para datos relacionales complejos. |
| **MongoDB** | Flexible para estructuras JSON, fácil de usar. | Mayor consumo de recursos, consultas menos optimizadas para datos temporales. |
| **PostgreSQL** | Relacional y con soporte para datos temporales a través de extensiones. | Complejidad y consumo de recursos mayor en hardware limitado. |
| **SQLite** | Muy ligera y simple. | No está optimizada para manejar grandes volúmenes de datos o análisis temporales avanzados. |