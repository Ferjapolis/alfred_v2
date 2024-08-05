#include <Wire.h>
#include <MPU6050.h>
#include <ESP8266WiFi.h>
#include <WiFiClient.h>

MPU6050 mpu;
const char* ssid = "NOMBRE_RED_WIFI"; // Reemplaza con el nombre de tu red WiFi
const char* password = "CONTRASEÑA_WIFI"; // Reemplaza con la contraseña de tu red WiFi
const char* host = "192.168.1.10"; // IP del otro ESP8266

void setup() {
  Serial.begin(115200);
  Wire.begin();
  mpu.initialize();
  
  // Conexión a WiFi
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("Conectado a WiFi");
}

void loop() {
  if (mpu.testConnection()) {
    int16_t ax, ay, az;
    mpu.getAcceleration(&ax, &ay, &az);

    // Conexión al servidor
    WiFiClient client;
    if (!client.connect(host, 80)) {
      Serial.println("Conexión fallida");
      return;
    }

    // Enviar datos
    String url = "/update?ax=" + String(ax) + "&ay=" + String(ay) + "&az=" + String(az);
    client.print(String("GET ") + url + " HTTP/1.1\r\n" +
                 "Host: " + host + "\r\n" + 
                 "Connection: close\r\n\r\n");
    delay(1000);
  } else {
    Serial.println("Fallo en la conexión del MPU6050");
  }
}