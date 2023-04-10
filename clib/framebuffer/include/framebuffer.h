#include <pistd>

typedef struct {
  uint32_t width;
  uint32_t height;
  uint32_t vwidth;
  uint32_t vheight;
  uint32_t bytes;
  uint32_t depth;
  uint32_t ignorex;
  uint32_t ignorey;
  void *pointer;
  uint32_t size;
} fb_init_t;