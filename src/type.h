#pragma once

typedef unsigned char u8;
typedef unsigned u32;

_Static_assert(sizeof(u8) == 1, "Size of u8 isn't 8 bits");
_Static_assert(sizeof(u32) == 4, "Size of u32 isn't 32 bits");
