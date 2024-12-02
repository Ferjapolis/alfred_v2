# Alfred v2 - Sistema Domótico

¡Bienvenido al repositorio de Alfred v2, tu solución integral para la gestión domótica de espacios inteligentes! Este proyecto combina hardware y software para ofrecer un sistema modular, eficiente y escalable que permite el monitoreo y control de dispositivos domésticos como sensores, relés y actuadores, todo a través de una interfaz web intuitiva.

## ¿Qué es Alfred v2?
### Alfred v2 es un ecosistema domótico basado en tecnologías modernas que consta de:

1. **[Backend en Rust](/backend/)**: Una API robusta y eficiente que gestiona la comunicación entre los nodos y la base de datos.
2. **[Frontend en Astro y Vue.js](/frontend/)**: Una interfaz web ligera que permite monitorear datos en tiempo real y controlar dispositivos desde cualquier navegador.
3. **[Nodos Arduino Mini + ESP8266](/nodos/)**: Microcontroladores responsables de recolectar datos de sensores (PIR, DHT22) y operar relés, conectados vía Wi-Fi.
4. **[Base de Datos MongoDB](/database/)**: Almacena de forma estructurada toda la información de sensores, eventos y estados de dispositivos.

## Arquitectura General del Sistema
A continuación, te mostramos cómo interactúan los componentes principales en Alfred v2:
![arquitectura](/img/arquitectura.png)

### Características Destacadas
- **Escalabilidad**: Fácil integración de más nodos ESP8266 para añadir sensores o dispositivos controlados.
- **Interfaz Moderna**: La web app permite interactuar con el sistema de forma intuitiva desde cualquier dispositivo conectado.
- **Almacenamiento Centralizado**: Los datos se almacenan en MongoDB, permitiendo consultas históricas y analíticas.
- **Automatización Domótica**: Control inteligente de relés y dispositivos basado en datos de sensores.

### Captura del Sistema en Acción
A continuación, se muestra un ejemplo de la comunicación entre un nodo ESP8266 y el servidor en Rust, acompañado del flujo de datos hacia la base de datos MongoDB:

![comunicacion](/img/comunicacion.png)

### ¿Cómo Empezar?
1. Clona este repositorio:
```bash
git clone https://github.com/tu_usuario/alfred-v2.git
```
2. Sigue las instrucciones de instalación y configuración en la documentación completa.

## Hardware
El servidor está diseñado para ejecutarse en una Raspberry Pi Zero 2 W, un dispositivo compacto y eficiente que ofrece conectividad inalámbrica, capacidad de procesamiento suficiente, y soporte para proyectos de bajo consumo.

### Raspberry Pi Zero 2 W ( [Manual](https://datasheets.raspberrypi.com/rpizero2/raspberry-pi-zero-2-w-product-brief.pdf) )

![RPIO_zero_2](https://assets.raspberrypi.com/static/f03a00485ac096c1033ad1c4a530d63b/15dad/zero2-close-up.webp)

### Specification

| Detalle | |
|---|---|
| **Form factor** | 65mm × 30mm |
| **Processor** | Broadcom BCM2710A1, quad-core 64-bit SoC (Arm Cortex-A53 @ 1GHz) |
| **Memory** | 512MB LPDDR2 |
| **Connectivity** | - 2.4GHz IEEE 802.11b/g/n wireless LAN, Bluetooth 4.2, BLE, |
| | onboard antenna |
| | 1 × USB 2.0 interface with OTG |
| | HAT-compatible 40-pin I/O header footprint |
| | microSD card slot |
| | Mini HDMI port |
| | CSI-2 camera connector |
| **Video** | HDMI interface |
| | Composite video
| **Multimedia** | H.264, MPEG-4 decode (1080p30) |
| | H.264 encode (1080p30)
| | OpenGL ES 1.1, 2.0 graphics
| **Input power** | 5V DC 2.5A |
| **Operating temperature** | -20°C to +70°C |
| **Production lifetime** | Raspberry Pi Zero 2 W will remain in production until  |at least
| | January 2030 |

Este hardware requiere un backend liviano y eficiente. Rust fue elegido debido a su capacidad para maximizar el rendimiento y reducir el consumo de recursos.
