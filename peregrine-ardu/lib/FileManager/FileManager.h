#pragma once
#include <FreeRTOS.h>
#include <queue.h>
#include <semphr.h>
#include <utility>

class FileManager {
 private:
  SemaphoreHandle_t mtx;
  QueueHandle_t queue;

 private:
  void task();
  static void startTask(void* _this);

 public:
  FileManager(/* args */);
  ~FileManager();
  bool begin();
  bool print(String f, String data);
};

namespace GLOBAL {
extern FileManager fileManager;
extern SemaphoreHandle_t VFSmtx;
}