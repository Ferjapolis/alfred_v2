#include <WiFi.h>
#include <Wire.h>
#include <PubSubClient.h>
#define acc 16384.0 //
#define Rad_grados 57.295779 //Conversión a radianes a grados 180/Pi

//----Instancias wifi y cliente MQTT----//
WiFiClient esp32Client; //Crea una instancia sin inicializar
PubSubClient mqttClient(esp32Client);

//----Parámetros de red----//
const char* ssid     = "Valhalla";
const char* password = "pfaW987h";

//----Parámetros MQTT----//
char* server = "broker.emqx.io";
int port = 1883;

int var=0;
int16_t AcX, AcY, AcZ;
float AngX;
char datos[40];
String resultS = "";

void wifiInit() {
    //Serial.print("Conectándose a ");
    //Serial.println(ssid);

    WiFi.begin(ssid, password);

    while (WiFi.status() != WL_CONNECTED) {
      //Serial.print(".");
        delay(500);  
    }
    //Serial.println("");
    //Serial.println("Conectado a WiFi");
    //Serial.println("Dirección IP: ");
    //Serial.println(WiFi.localIP());
  }

//----Callback de suscripción----//
void callback(char* topic, byte* payload, unsigned int length) {
  //Serial.print("Mensaje recibido [");
  //Serial.print(topic);
  //Serial.print("] ");

//Para utilizar la info del callback como un nro se deben declarar las siguientes variables//

  char payload_string[length + 1]; //Para tener suficiente espacio
  
  int resultI;

  memcpy(payload_string, payload, length); //Se indica dónde se guardará, de dónde se sacan y el tamaño
  payload_string[length] = '\0';
  resultI = atoi(payload_string); //Para convertir los datos enteros o flotantes
  
 //Esos datos se mueven a la variable 'var' para poder utilizarlos en el programa
 
 var = resultI; //ver variable topic del ejemplo 10, curso 'Nod-RED Básico II'

  resultS = "";

  //El siguiente "For" es para obtener los datos en un "string"
  for (int i=0;i<length;i++) {
    resultS= resultS + (char)payload[i];
  }
  //Serial.println();
}


//Comprobación de conexión del cliente
void reconnect() {
  while (!mqttClient.connected()) { //Comprueba si el cliente está conectado al Broker
    //Serial.print("Intentando conectarse MQTT...");

    if (mqttClient.connect("arduinoClient")) {
      //Serial.println("Conectado");

      mqttClient.subscribe("Tanque/Nivel"); //Se suscribe a los payload publicados sobre el topic específico.
      //Parámetros,
      //topic - const char []: El topic al que se suscribe.
      //qos - int (0 o 1): La calidad de servicio.

    } else {       //Cliente no conectado reintentando conexión en 5 seg.
      //Serial.print("Fallo, rc=");
      //Serial.print(mqttClient.state());
      //Serial.println(" intentar de nuevo en 5 segundos");
      // Wait 5 seconds before retrying
      delay(5000);
    }
  }
}


/*Si el cliente se configura como SUSCRIPTOR, se debe proporcionar una callback 
en el constructor. Esta función se llamará cuando llegan nuevos mensajes al cliente. 
Consultar líena 37.*/

/*void callback(const char[] topic, byte* payload, unsigned int length)
  void callback(char* topic, byte* payload, unsigned int length){
   String payload_,
      for(int i=0;i<lenght;i++){
        payload_+= (char)payload[i];
      }
}*/

void setup()
{
  Wire.begin();
  Wire.beginTransmission(0x68);
  Wire.write(0x6B);
  Wire.write(0); //Al enviar cero, le indicamos que se active. Que salga de modo sleep
  Wire.endTransmission(true);
  
  wifiInit();
  mqttClient.setServer(server, port);
  mqttClient.setCallback(callback);
}

void loop()
{
   if (!mqttClient.connected()) {
    reconnect();
  }
  
  //if(!mqttClient.loop()) mqttClient.connect("arduinoClient"); //Función de la librería SubPubClient. 
  //La función 'loop()' debe llamarse con frecuencia para permitir que el cliente procese los mensajes entrantes y mantenga su conexión con el Broker.

  mqttClient.loop(); //Función de la librería SubPubClient. 
 
  Wire.beginTransmission(0x68);
  Wire.write(0x3B); //Dirección de variable de acelerómetros
  Wire.endTransmission(false);
  Wire.requestFrom(0x68, 6, true); //A partir del 0x3B, se piden 6 registros
  AcX = Wire.read() << 8 | Wire.read();
  AcY = Wire.read() << 8 | Wire.read();
  AcZ = Wire.read() << 8 | Wire.read();
 
  AngX = atan((AcZ / acc) / ((AcX / acc) * sqrt(2))) * Rad_grados; //Ecuación para inclinación en radianes

  sprintf(datos, "Valor Sensor: %d ", AngX);
  mqttClient.publish("Tanque/Nivel",datos); //Averiguar si es posible publicar datos de este modo o como string
  delay(5000);
  
}
