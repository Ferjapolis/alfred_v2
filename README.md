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