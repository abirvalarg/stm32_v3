#include "type.h"

extern u32 _STACK;

void reset();
__attribute__((weak, alias("default_handler")))
void NMI();
__attribute__((weak, alias("default_handler")))
void hardfault();

__attribute__((section(".rodata.vector")))
void *vector[] = {
	&_STACK, reset, NMI, hardfault
};

void default_handler()
{
	while(1)
		asm("wfi");
}
