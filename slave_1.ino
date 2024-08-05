#include <ESP8266WiFi.h>
#include <WiFiServer.h>
#include <WiFiClient.h>
#include <SoftwareSerial.h>

const char* ssid = "NOMBRE_RED_WIFI"; // Reemplaza con el nombre de tu red WiFi
const char* password = "CONTRASEÑA_WIFI"; // Reemplaza con la contraseña de tu red WiFi

WiFiServer server(80);
SoftwareSerial mySerial(D1, D2); // Pines para comunicación con Arduino Uno (RX, TX)

void setup() {
  Serial.begin(115200);
  mySerial.begin(9600);

  // Conexión a WiFi
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("Conectado a WiFi");

  // Iniciar el servidor
  server.begin();
  Serial.println("Servidor iniciado");
}

void loop() {
  WiFiClient client = server.available();
  if (client) {
    String req = client.readStringUntil('\r');
    Serial.println(req);
    client.flush();

    // Parsear los datos recibidos
    if (req.indexOf("/update?") != -1) {
      int axStart = req.indexOf("ax=") + 3;
      int ayStart = req.indexOf("&ay=") + 4;
      int azStart = req.indexOf("&az=") + 4;

      int ax = req.substring(axStart, req.indexOf("&ay=")).toInt();
      int ay = req.substring(ayStart, req.indexOf("&az=")).toInt();
      int az = req.substring(azStart).toInt();

      // Enviar datos al Arduino Uno
      mySerial.print("ax:");
      mySerial.print(ax);
      mySerial.print(" ay:");
      mySerial.print(ay);
      mySerial.print(" az:");
      mySerial.println(az);

      // Decidir el control de la bomba
      if (ax < LIMITE_INFERIOR || ay < LIMITE_INFERIOR || az < LIMITE_INFERIOR) {
        mySerial.println("ENCENDER_BOMBA");
      } else {
        mySerial.println("APAGAR_BOMBA");
      }
    }
    client.stop();
  }
}


#include <SoftwareSerial.h>

SoftwareSerial mySerial(2, 3); // Pines de comunicación con ESP8266 (RX, TX)

const int bombaPin = 7; // Pin para controlar el relé de la bomba

void setup() {
  pinMode(bombaPin, OUTPUT);
  digitalWrite(bombaPin, LOW); // Apagar la bomba al inicio
  Serial.begin(9600);
  mySerial.begin(9600);
}

void loop() {
  if (mySerial.available()) {
    String comando = mySerial.readStringUntil('\n');
    Serial.println("Comando recibido: " + comando);

    if (comando == "ENCENDER_BOMBA") {
      digitalWrite(bombaPin, HIGH); // Encender la bomba
      Serial.println("Bomba encendida");
    } else if (comando == "APAGAR_BOMBA") {
      digitalWrite(bombaPin, LOW); // Apagar la bomba
      Serial.println("Bomba apagada");
    }
  }
}
