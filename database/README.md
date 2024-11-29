### Sistema Domótico Alfred v2
# Database

### Descripción General
La base de datos utilizada en el proyecto Alfred v2 es MongoDB, una base de datos NoSQL que almacena datos en formato JSON. La estructura está diseñada para soportar la interacción entre los nodos ESP8266, el servidor Rust y la interfaz web.

La base de datos se llama `domotica` y contiene las siguientes colecciones principales:

- `sensors`: Almacena lecturas de temperatura y humedad enviadas por los nodos.
- `reles`: Registra el estado de los relés controlados desde los nodos o la web.
- `pir`: Guarda activaciones de sensores de movimiento (PIR) de los nodos.


### Modelos de Datos
#### 1. Modelo sensors
Cada documento de la colección sensors representa una lectura de temperatura y humedad.
**Estructura del documento**:

```json
{
  "_id": "ObjectId",
  "nodo": "String",           // ID del nodo que envió los datos
  "temperature": "Float",    // Temperatura medida en °C
  "humidity": "Float",       // Humedad medida en %
  "timestamp": "ISODate"     // Fecha y hora de la lectura
}
```
Ejemplo:

```json
{
  "_id": "642c4ae88d9f04c89b8c9e13",
  "nodo": "N001",
  "temperature": 23.5,
  "humidity": 60.2,
  "timestamp": "2024-11-28T14:30:00.000Z"
}
```

#### 2. Modelo reles
Cada documento de la colección reles representa un cambio de estado de un relé.
**Estructura del documento**:

```json
{
  "_id": "ObjectId",
  "nodo": "String",          // ID del nodo que controló el relé
  "relay": "Integer",        // Número del relé (0-3)
  "state": "Boolean",        // Estado del relé (true=encendido, false=apagado)
  "timestamp": "ISODate"     // Fecha y hora del cambio de estado
}
```

Ejemplo:
```json
{
  "_id": "642c4b058d9f04c89b8c9e14",
  "nodo": "N001",
  "relay": 2,
  "state": true,
  "timestamp": "2024-11-28T14:35:00.000Z"
}
```

#### 3. Modelo pir
Cada documento de la colección pir representa la detección de movimiento por un sensor PIR.
**Estructura del documento**:

```json
{
  "_id": "ObjectId",
  "nodo": "String",          // ID del nodo que reportó el movimiento
  "pir": "Integer",          // Número del sensor PIR (0-2)
  "state": "Boolean",        // Estado del PIR (true=detectado, false=no detectado)
  "timestamp": "ISODate"     // Fecha y hora de la detección
}
```
Ejemplo:
```json
{
  "_id": "642c4b3a8d9f04c89b8c9e15",
  "nodo": "N001",
  "pir": 1,
  "state": true,
  "timestamp": "2024-11-28T14:40:00.000Z"
}
```
### Diagrama de Colecciones
Aquí tienes un diagrama visual que muestra cómo se relacionan las colecciones y sus campos principales:

![erd](/img/erd.png)

### Consultas Comunes
1. Obtener las lecturas de temperatura y humedad de las últimas 24 horas
```javascript
db.sensors.find({
  "timestamp": { "$gte": new ISODate("2024-11-27T14:30:00.000Z") }
});
```

2. Obtener los cambios de estado de los relés de la última semana
```javascript
db.reles.find({
  "timestamp": { "$gte": new ISODate("2024-11-21T14:30:00.000Z") }
});
```

3. Obtener todas las detecciones de movimiento de un nodo específico
```javascript
db.pir.find({ "nodo": "N001" });
```
