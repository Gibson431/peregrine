#include <Arduino.h>
#include <Debug.h>
#include <FileManager.h>
#include <FreeRTOS.h>
#include <LittleFS.h>
#include <VFS.h>
#include <task.h>

void blink(void* pvParams) {
  int period = (int)pvParams;
  pinMode(LED_BUILTIN, OUTPUT);
  for (;;) {
    vTaskDelay(1000 / portTICK_PERIOD_MS);
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
  }
}

void startup(void* pvParams) {
  GLOBAL::fileManager.begin();
  DBG("startup complete");
}

void config(void* pvParams) {
  xTaskCreate(blink, "blink", 1000, (void*)1000, 0, NULL);

  for (;;) {
    GLOBAL::fileManager.print("/sd/test.txt", "asdf\n");
    DBG("asdf");
    vTaskDelay(1000 / portTICK_PERIOD_MS);
  }
}

void setup() {
  Serial.begin(9600);
  vTaskDelay(1000 / portTICK_PERIOD_MS);

  // async startup procedure
  xTaskCreate(startup, "startup", 1000, NULL, 0, NULL);

  // delay for time to connect to computer
  vTaskDelay(5000 / portTICK_PERIOD_MS);

  if (Serial) {
    Serial.println("Connected to host, entering config mode...");
    vTaskDelay(1000 / portTICK_PERIOD_MS);
    xTaskCreate(config, "config", 1000, NULL, 0, NULL);
  } else {
    xTaskCreate(blink, "blink", 1000, (void*)1000, 0, NULL);
  }

  // delete default task
  vTaskDelete(NULL);
}

void loop() {
  // Empty loop, this task was deleted
}
