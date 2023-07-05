/*
 * Generated with the FPGA Interface C API Generator 19.0
 * for NI-RIO 19.0 or later.
 */
#ifndef __NiFpga_Main_h__
#define __NiFpga_Main_h__

#ifndef NiFpga_Version
   #define NiFpga_Version 190
#endif

#include "NiFpga.h"

/**
 * The filename of the FPGA bitfile.
 *
 * This is a #define to allow for string literal concatenation. For example:
 *
 *    static const char* const Bitfile = "C:\\" NiFpga_Main_Bitfile;
 */
#define NiFpga_Main_Bitfile "NiFpga_Main.lvbitx"

/**
 * The signature of the FPGA bitfile.
 */
static const char* const NiFpga_Main_Signature = "E3E0C23C5F01C0DBA61D947AB8A8F489";

#if NiFpga_Cpp
extern "C"
{
#endif

typedef enum
{
   NiFpga_Main_IndicatorU8_U8Result = 0x1800A,
} NiFpga_Main_IndicatorU8;

typedef enum
{
   NiFpga_Main_ControlU8_U8Control = 0x18002,
   NiFpga_Main_ControlU8_U8Sum = 0x18006,
} NiFpga_Main_ControlU8;

typedef enum
{
   NiFpga_Main_IndicatorArrayU8_U8ResultArray = 0x1800C,
} NiFpga_Main_IndicatorArrayU8;

typedef enum
{
   NiFpga_Main_IndicatorArrayU8Size_U8ResultArray = 4,
} NiFpga_Main_IndicatorArrayU8Size;

typedef enum
{
   NiFpga_Main_ControlArrayU8_U8ControlArray = 0x18014,
   NiFpga_Main_ControlArrayU8_U8SumArray = 0x18010,
} NiFpga_Main_ControlArrayU8;

typedef enum
{
   NiFpga_Main_ControlArrayU8Size_U8ControlArray = 4,
   NiFpga_Main_ControlArrayU8Size_U8SumArray = 4,
} NiFpga_Main_ControlArrayU8Size;


#if NiFpga_Cpp
}
#endif

#endif
