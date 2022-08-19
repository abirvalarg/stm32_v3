#include "type.h"

extern u32 _STACK;

void reset();
__attribute__((weak, alias("default_handler")))
void NMI();
__attribute__((weak, alias("default_handler")))
void hardfault();
__attribute__((weak, alias("default_handler")))
void TIM3();

__attribute__((section(".vector")))
void *vector[] = {
	&_STACK, reset, NMI, hardfault,
	0, 0, 0, 0,
	0, 0, 0, 0,
	0, 0, 0, 0,
	// 0
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	// 10
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	// 20
	0, 0, 0, 0, 0, 0, 0, 0, 0, TIM3,
};

void default_handler()
{
	while(1)
		asm("wfi");
}
