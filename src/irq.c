#include "type.h"

extern u32 _STACK;

void reset();
__attribute__((weak, alias("default_handler")))
void NMI();
__attribute__((weak, alias("default_handler")))
void hardfault();

#ifdef F4
void TIM3();
void TIM4();

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
	// 30
	TIM4
};

#endif

void default_handler()
{
	while(1);
}
