#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <DHT.h>

// Configuración de Wi-Fi
const char* ssid = "tu_SSID";
const char* password = "tu_contraseña";

// Configuración de sensores y relés
#define DHTPIN 2     // Pin donde está conectado el DHT22
#define DHTTYPE DHT22
DHT dht(DHTPIN, DHTTYPE);

const int pirPins[] = {12, 13, 14}; // Pines donde están conectados los sensores PIR
const int relayPins[] = {5, 4, 0, 2}; // Pines donde están conectados los relés
const int buttonPins[] = {10, 9, 8, 7}; // Pines donde están conectados los botones

// Configuración del servidor
const char* serverName = "http://IP_de_tu_Raspberry/api/endpoint";
const char* nodeId = "N001"; // Identificador del nodo

ESP8266WebServer server(80); // Servidor HTTP en el puerto 80

void setup() {
  Serial.begin(115200);
  dht.begin();

  // Configurar pines de relés y botones
  for (int i = 0; i < 4; i++) {
    pinMode(relayPins[i], OUTPUT);
    pinMode(buttonPins[i], INPUT_PULLUP);
  }

  // Configurar pines de sensores PIR
  for (int i = 0; i < 3; i++) {
    pinMode(pirPins[i], INPUT);
  }

  // Conectar a Wi-Fi
  WiFi.begin(ssid, password);
  Serial.println("Conectando a Wi-Fi...");
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Conectando...");
  }
  Serial.println("Conectado a Wi-Fi");

  // Configurar rutas del servidor HTTP
  server.on("/sensors", HTTP_GET, handleGetSensors);
  server.begin();
  Serial.println("Servidor HTTP iniciado");
}

void loop() {
  // Manejar solicitudes del servidor HTTP
  server.handleClient();

  // Leer botones y operar relés
  for (int i = 0; i < 4; i++) {
    if (digitalRead(buttonPins[i]) == LOW) {
      bool newState = !digitalRead(relayPins[i]);
      digitalWrite(relayPins[i], newState);

      // Enviar registro de cambio de estado del relé al servidor
      if (WiFi.status() == WL_CONNECTED) {
        HTTPClient http;
        String serverPath = serverName + "/reles";
        String jsonPayload = "{\"nodo\":\"" + String(nodeId) + "\",\"relay\":" + String(i) + ",\"state\":" + String(newState) + "}";

        http.begin(serverPath.c_str());
        http.addHeader("Content-Type", "application/json");
        int httpResponseCode = http.POST(jsonPayload);

        if (httpResponseCode > 0) {
          String response = http.getString();
          Serial.println(httpResponseCode);
          Serial.println(response);
        } else {
          Serial.print("Error en la solicitud HTTP: ");
          Serial.println(httpResponseCode);
        }
        http.end();
      }

      delay(300); // Debounce
    }
  }

  // Registrar activación de sensores PIR
  for (int i = 0; i < 3; i++) {
    bool pirValue = digitalRead(pirPins[i]);
    if (pirValue) {
      if (WiFi.status() == WL_CONNECTED) {
        HTTPClient http;
        String serverPath = serverName + "/pir";
        String jsonPayload = "{\"nodo\":\"" + String(nodeId) + "\",\"pir\":" + String(i) + ",\"state\":" + String(pirValue) + "}";

        http.begin(serverPath.c_str());
        http.addHeader("Content-Type", "application/json");
        int httpResponseCode = http.POST(jsonPayload);

        if (httpResponseCode > 0) {
          String response = http.getString();
          Serial.println(httpResponseCode);
          Serial.println(response);
        } else {
          Serial.print("Error en la solicitud HTTP: ");
          Serial.println(httpResponseCode);
        }
        http.end();
      }
    }
  }

  delay(2000);
}

void handleGetSensors() {
  // Leer sensores
  float h = dht.readHumidity();
  float t = dht.readTemperature();
  bool pirValues[3];
  for (int i = 0; i < 3; i++) {
    pirValues[i] = digitalRead(pirPins[i]);
  }

  // Crear respuesta JSON
  String jsonResponse = "{\"nodo\":\"" + String(nodeId) + "\",\"temperature\":" + String(t) + ",\"humidity\":" + String(h)  + "}";

  // Enviar respuesta JSON
  server.send(200, "application/json", jsonResponse);
}
