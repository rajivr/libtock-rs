/* Layout for the Hail board, used by the examples in this repository. */

MEMORY {
  /* The application region is 64 bytes (0x40) */
  FLASH (rx) : ORIGIN = 0x00030040, LENGTH = 0x0005FFC0
  SRAM (rwx) : ORIGIN = 0x20004000, LENGTH = 62K
}

MPU_MIN_ALIGN = 8K;

INCLUDE layout.ld
