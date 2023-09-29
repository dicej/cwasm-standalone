#include <stdint.h>
#include <stdio.h>

extern int32_t run_wasm_native_binary(size_t len, uint8_t *guest);

int main() {
  extern const char guest[];
  extern const size_t guest_len;
  int32_t result = run_wasm_native_binary(guest_len, (uint8_t*)guest);
  return result;
}
