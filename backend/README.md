### Sistema Domótico Alfred v2
# Backend
Este proyecto es un backend desarrollado en Rust utilizando el framework Actix Web. Está diseñado para gestionar un sistema domótico que interactúa con sensores, relés, y detectores PIR. Los datos se almacenan en una base de datos MongoDB, permitiendo la consulta y registro de información para su análisis y control.

## Funcionalidades Principales
1. Registrar datos de sensores (DHT22):
    - Registra lecturas de temperatura y humedad por nodo.
    - Endpoint: `POST /sensors`.
2. Registrar estado de relés:
    - Guarda el estado (encendido/apagado) de un relé específico en un nodo.
    - Endpoint: `POST /reles`.
3. Registrar estado de sensores PIR:
    - Registra activaciones de sensores PIR por nodo.
    - Endpoint: `POST /pir`.
4. Obtener datos históricos:
    - Consulta datos registrados en periodos como "día", "semana", o "mes".
    - Endpoint genérico: `GET /{table}/{period}` (donde `table` es `sensors`, `reles` o `pir`).

### Estructura del Código
1. **Modelos de datos**:
    - `SensorData`: Define la estructura de datos enviados desde un sensor DHT22.
    - `RelayData`: Representa la información de un relé (nodo, número de relé, estado).
    - `PirData`: Define los datos de activación de sensores PIR.
    - `SensorRecord`, `RelayRecord`, `PirRecord`: Estructuras para almacenar los datos en la base de datos, incluyendo una marca de tiempo (`timestamp`).

2. **Endpoints principales**:
    - `save_sensor_data`: Registra datos de temperatura y humedad.
    - `save_relay_data`: Registra cambios de estado en los relés.
    - `save_pir_data`: Registra activaciones de sensores PIR.
    - `get_data`: Consulta datos almacenados filtrados por tiempo (day, week, month).

3. **Base de datos**:
    - Utiliza MongoDB para persistir los datos.
    - Colecciones:
        - `sensors`: Almacena datos de sensores de temperatura y humedad.
        - `reles`: Almacena estados de relés.
        - `pir`: Almacena estados de sensores PIR.

4. **Servidor HTTP**:
    - El servidor escucha en el puerto 8080 y expone los endpoints a través de rutas.

### Detalles Técnicos
1. **Dependencias principales**:
    - **Actix Web**: Framework para el desarrollo de aplicaciones web asíncronas en Rust.
    - **MongoDB Driver**: Interacción con la base de datos.
    - **Serde**: Serialización y deserialización de estructuras de datos.
    - **Chrono**: Gestión de fechas y horas.
    - **Futures**: Procesamiento asíncrono de flujos de datos.

2. **Gestión de tiempos**:
    - Se utilizan las funciones de Chrono para calcular filtros temporales (day, week, month).

3. **Modelo asíncrono**:
    - Todas las operaciones de base de datos y comunicación HTTP se ejecutan de manera asíncrona utilizando async y await.

### Endpoints Explicados
| Método | Ruta | Descripción |
|---|---|---|
| POST | `/sensors` | Registra datos de temperatura y humedad. |
| POST | `/reles` | Guarda el estado de un relé. |
| POST | `/pir` | Registra el estado de un sensor PIR. |
| GET | `/{table}/{period}` | Consulta datos históricos filtrados por periodo. |

### Diagrama de comunicaciones
#### Backend a Frontend
![conexion](/img/server-frontend.png)

#### Backend a Nodos
![conexion](/img/server-nodos.png)

### Ejemplo de Peticiones
- Registrar datos de un sensor:   
    ```bash
    curl -X POST -H "Content-Type: application/json" \
    -d '{"nodo":"nodo1", "temperature":22.5, "humidity":60.2}' \
    http://localhost:8080/sensors
    ```

2. Registrar el estado de un relé:
    ```bash
    curl -X POST -H "Content-Type: application/json" \
    -d '{"nodo":"nodo1", "relay":2, "state":true}' \
    http://localhost:8080/reles
    ```

- Consultar datos del último día:
    ```bash
    curl -X GET http://localhost:8080/sensors/day
    ```

### Cómo Ejecutar
1. Asegúrate de tener una instancia de MongoDB en ejecución en localhost:27017.
2. Compila y ejecuta el servidor:
    ```bash
    cargo run
    ```
3. Accede a los endpoints usando herramientas como Postman, curl, o una aplicación cliente personalizada.
