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
static const char* const NiFpga_Main_Signature = "728411ED7A6557687BCF28DB1D70ACF2";

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


#if NiFpga_Cpp
}
#endif

#endif
