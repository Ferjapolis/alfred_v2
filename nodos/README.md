### Sistema Domótico Alfred v2
# Nodos

Este código está diseñado para ejecutarse en un microcontrolador ESP8266 que actúa como nodo en el sistema domótico. Cada nodo es responsable de manejar sensores (DHT22 y PIR), relés, y botones físicos. Además, se conecta a una red Wi-Fi y se comunica con un servidor (backend) para enviar datos y recibir instrucciones.

## Funcionalidades Principales
1. Lectura de Sensores:
    - Lee datos de temperatura y humedad del sensor DHT22.
    - Detecta activaciones de sensores PIR (infrarrojo pasivo) conectados al nodo.
2. Control de Relés:
    - Cambia el estado de los relés mediante botones físicos.
    - Notifica al servidor cada cambio de estado en los relés.
3. Comunicación con el Servidor:
    - Envío de datos de sensores al backend para su registro.
    - Reporta activaciones de sensores PIR.
    - Sincroniza los cambios de estado de los relés con el backend.

### Configuración de Hardware
- **Sensor DHT22**:
    - Conectado al pin D2.
- **Sensores PIR**:
    - Conectados a los pines D12, D13, y D14.
- **Relés**:
    - Conectados a los pines D5, D4, D0, y D2.
- **Botones**:
    - Conectados a los pines D10, D9, D8, y D7.

### Configuración de Red
- **SSID**: Nombre de la red Wi-Fi.
- **Contraseña**: Contraseña de la red Wi-Fi.
- **Servidor**: Dirección IP del servidor backend.
- **Identificador del Nodo**: Cada nodo tiene un identificador único (`nodeId`), por ejemplo, `N001`.

### Ciclo de Vida del Nodo
1. **Inicio**:
    - Se conecta a la red Wi-Fi.
    - Configura los pines para los sensores, relés y botones.
2. **Bucle Principal**:
    - Lectura y envío de datos del DHT22:
        - Lee la temperatura y la humedad.
        - Envía los datos al endpoint /sensors del servidor.
    - Gestión de botones:
        - Detecta si algún botón es presionado.
        - Cambia el estado del relé correspondiente.
        - Envía el cambio de estado al servidor en el endpoint /reles.
    - Monitoreo de sensores PIR:
        - Detecta activaciones de los sensores PIR.
        - Reporta estas activaciones al servidor en el endpoint /pir.
3. **Esperas**:
    - Retraso de 2000 ms en cada ciclo para evitar saturar la red y el servidor.

### Estructura de Comunicación con el Servidor
- Protocolo HTTP:
    - Las peticiones se realizan con el cliente HTTP integrado en la librería ESP8266HTTPClient.
    - Cada solicitud incluye un payload en formato JSON.

| Endpoint | Método | Descripción | Ejemplo de Payload |
|---|---|---|---|
| `/sensors` | POST | Envía datos de temperatura y humedad. | `{"nodo":"N001","temperature":22.5,"humidity":60.2}` |
| `/reles` | POST | Reporta cambios en el estado de un relé. | `{"nodo":"N001","relay":1,"state":true}` |
| `/pir` | POST | Reporta activaciones de un sensor PIR. | `{"nodo":"N001","pir":0,"state":true}` |

![conexion](/img/conexion.png)

### Ejemplo de Payloads Enviados
1. **Lectura de Sensor DHT22**:
```json
{
  "nodo": "N001",
  "temperature": 23.4,
  "humidity": 55.8
}
```

2. **Cambio de Estado en un Relé**:
```json
{
  "nodo": "N001",
  "relay": 2,
  "state": false
}
```

3. **Activación de un Sensor PIR**:
```json
{
  "nodo": "N001",
  "pir": 1,
  "state": true
}
```

### Detalles del Código
1. **Conexión a Wi-Fi**:
    - La conexión a Wi-Fi se establece en el bloque setup().
    - Se verifica continuamente el estado de conexión con WiFi.status().
2. **Lectura de Sensores**:
    - El sensor DHT22 se inicializa con la librería DHT.h.
    - Los valores de los sensores PIR se leen mediante digitalRead().
3. **Control de Relés**:
    - Los relés se controlan invirtiendo su estado actual cada vez que se presiona un botón.
    - Un retardo de 300 ms (delay(300)) se utiliza para evitar rebotes.
4. **Envío de Datos al Servidor**:
    - Se usan peticiones HTTP POST para enviar datos al servidor.
    - Los encabezados incluyen Content-Type: application/json.
    - La respuesta del servidor se imprime en el puerto serie para depuración.

### Cómo Probar
1. Configura la red Wi-Fi y la dirección del servidor en el código.
2. Sube el código al ESP8266 usando Arduino IDE.
3. Monitorea el puerto serie para verificar las lecturas y las respuestas del servidor.