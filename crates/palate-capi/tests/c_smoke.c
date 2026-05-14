#include "palate.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  const char *name = NULL;
  const uint8_t content[] = "fn main() {}\n";

  palate_status_t status =
      palate_try_detect("main.rs", content, sizeof(content) - 1, &name);
  if (status != PALATE_STATUS_OK) {
    fprintf(stderr, "unexpected status: %d\n", status);
    return 1;
  }
  if (name == NULL || strcmp(name, "rust") != 0) {
    fprintf(stderr, "unexpected file type: %s\n", name == NULL ? "(null)" : name);
    return 2;
  }

  return 0;
}
