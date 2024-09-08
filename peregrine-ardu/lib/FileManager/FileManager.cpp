#include <Arduino.h>
#include <FileManager.h>
#include <FreeRTOS.h>
#include <queue.h>
#include <semphr.h>

#include <Debug.h>
#include <LittleFS.h>
#include <SDFS.h>
#include <VFS.h>
#include <task.h>
#include <utility>

FileManager::FileManager() {
  this->mtx = xSemaphoreCreateMutex();
  this->queue = xQueueCreate(16, sizeof(std::pair<String*, String*>));
}

bool FileManager::begin() {
  xSemaphoreTake(this->mtx, portMAX_DELAY);

  // mount littlefs
  if (LittleFS.begin()) {
    VFS.map("/lfs", LittleFS);
    DBG("LittleFS\t[OK]")
  } else {
    DBG("LittleFS\t[ERROR]")
    return false;
  };

  // mount sdfs
  if (SDFS.begin()) {
    DBG("SDFS\t[OK]")
    VFS.map("/sd", SDFS);
  } else {
    DBG("SDFS\t[ERROR]")
  };

  xSemaphoreGive(this->mtx);

  xTaskCreate(this->startTask, "fs", 2048, this, 1, NULL);

  return true;
};

void FileManager::startTask(void* _this) {
  static_cast<FileManager*>(_this)->task();
  vTaskDelete(NULL);
}

void FileManager::task() {
  DBG("File Manager\t[OK]")
  std::pair<String*, String*> p;
  for (;;) {
    xQueueReceive(this->queue, &p, portMAX_DELAY);
    FILE* f = fopen((*p.first).c_str(), "a");
    if (f) {
      fprintf(f, (*p.second).c_str());
      fclose(f);
    } else {
      DBG("Error opening " + *p.first);
    }

    delete p.first;
    delete p.second;
  }
}

/// @brief Sends a string to the file queue to be processed later
/// @param f The file to send data to
/// @param data The data
/// @return True if the data was successfully sent to the file queue
bool FileManager::print(String f, String data) {
  String* fPtr = new String(f);
  String* dataPtr = new String(data);
  std::pair<String*, String*> p = std::make_pair(fPtr, dataPtr);
  return xQueueSend(this->queue, &p, (TickType_t)0);
};

namespace GLOBAL {
FileManager fileManager;
}  // namespace GLOBAL