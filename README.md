# Alfred v2 - Sistema Domótico

### Descripción
Alfred v2 es un sistema domótico diseñado para automatizar y controlar dispositivos dentro de una casa inteligente. Utiliza un ecosistema basado en hardware como Raspberry Pi Zero 2 W y placas Arduino con ESP8266, acompañado de un stack tecnológico moderno que incluye MongoDB, Rust, Astro y Vue.js.


## Tabla de Contenidos
- Arquitectura del Sistema
- Hardware
    - Raspberry Pi Zero 2 W
    - Arduino con ESP8266
- Software
    - Base de Datos
    - Backend
    - Frontend
- Instalación
- Uso
- Estructura del Repositorio
- Contribuciones
- Licencia

## Arquitectura del Sistema
El sistema Alfred v2 se compone de los siguientes elementos:
![diagrama](/diagrama_de_arquitectura.png)
- Comunicación entre los ESP8266 y el backend mediante MQTT o HTTP.
- Base de datos MongoDB para registros y configuración.
- Interfaz de usuario para lectura y control en tiempo real.

### Server
Raspberry Pi Zero 2 W: Hospeda el backend, la base de datos y la aplicación web.

1. **Backend**
    - Rust: Microservicio backend que gestiona la lógica del sistema y comunicación.

2. **Base de Datos**
    - MongoDB: Base de datos para almacenar estados de sensores y registros históricos.

3. **Frontend**
    - Astro & Vue.js: Aplicación frontend para interactuar con el usuario.
    - Node.js: Servidor para la aplicación web.

### Nodos
Arduino Mini + ESP8266: Controla sensores y actuadores para interactuar con el entorno físico.
Tecnologías.

1. **Input**:
    - PIR (sensores de movimientos)
    - DHT22 (sensor de humedad y temperatura)
    - BMP280 (sensor de presion atmoferica)
2. Output
    - Rele
    - Leds

