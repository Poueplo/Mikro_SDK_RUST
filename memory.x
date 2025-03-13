/* memory.x - Linker script for the STM32F429ZI */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 2mB*/
  FLASH : ORIGIN = 0x08000000, LENGTH = 2000K
  /* RAM begins at 0x20000000 and has a size of 112kB*/
  RAM : ORIGIN = 0x20000000, LENGTH = 112K
}