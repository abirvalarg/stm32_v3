#include "type.h"

#ifdef HEAP
	void heap_init();
#endif

#pragma GCC diagnostic ignored "-Wmain-return-type"
void main();
#ifdef ASYNC
__attribute__((noreturn))
void executor_loop();
#endif

extern u8 _DATA_START, _DATA_END, _DATA_VAL_START, _BSS_START, _BSS_END;

__attribute__((cold, noreturn))
void reset()
{
	for(
		u8 *src = &_DATA_VAL_START, *dest = &_DATA_START;
		dest < &_DATA_END;
		src++, dest++
	)
		*dest = *src;
	
	for(
		u8 *dest = &_BSS_START;
		dest < &_BSS_END;
		dest++
	)
		*dest = 0;
	
#ifdef HEAP
	heap_init();
#endif

	main();
#ifdef ASYNC
	executor_loop();
#else
	while(1)
		asm("wfi");
#endif
}
