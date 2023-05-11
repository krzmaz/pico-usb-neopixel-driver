#include <pico/stdlib.h>

#include <iostream>
#include <optional>
#include <vector>

#include "log.h"
#include "ws2812.pio.h"

#define WS2812_PIN 2
#define IS_RGBW false

#define MAX_PIXELS 1500
#define MAX_BYTES (MAX_PIXELS * 3) + 2

static inline void put_pixel(uint32_t pixel_grb) {
  pio_sm_put_blocking(pio0, 0, pixel_grb << 8u);
}

static inline uint32_t urgb_u32(uint8_t r, uint8_t g, uint8_t b) {
  return ((uint32_t)(r) << 8) | ((uint32_t)(g) << 16) | (uint32_t)(b);
}

void init() {
  stdio_init_all();
  PIO pio = pio0;
  int sm = 0;
  uint offset = pio_add_program(pio, &ws2812_program);
  ws2812_program_init(pio, sm, offset, WS2812_PIN, 800000, IS_RGBW);
}

int main() {
  init();
  
  std::vector<unsigned char> vec;
  vec.reserve(MAX_BYTES);
  std::optional<unsigned short> length;
  while (true) {
    auto ch = getchar();
    if (ch >= 0) {
      LOG("got char: ");
      LOG(ch);
      LOG("whole vec: ");
      LOG(vec);

      vec.push_back(static_cast<unsigned char>(ch));
      if (!length) {
        if (vec.size() > 1) {
          length = vec[0] + (((unsigned char)vec[1]) << 8);
          LOG("length set: ");
          LOG(*length);
          vec.clear();
        }
        continue;
      }
      if (vec.size() == *length) {
        LOG("Got enough data!");
        for (size_t i = 0; i < vec.size(); i += 3) {
          auto val = urgb_u32(vec[i], vec[i + 1], vec[i + 2]);
          put_pixel(val);
        }
        vec.clear();
        length = std::nullopt;
      }
    }
  }
  return 0;
}