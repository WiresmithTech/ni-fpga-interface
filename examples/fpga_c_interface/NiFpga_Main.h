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
static const char* const NiFpga_Main_Signature = "A0613989B20F45FC6E79EB71383493E8";

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
   NiFpga_Main_IndicatorU32_IRQs = 0x18060,
} NiFpga_Main_IndicatorU32;

typedef enum
{
   NiFpga_Main_IndicatorSgl_SglResult = 0x18024,
} NiFpga_Main_IndicatorSgl;

typedef enum
{
   NiFpga_Main_ControlU8_U8Control = 0x18002,
   NiFpga_Main_ControlU8_U8Sum = 0x18006,
} NiFpga_Main_ControlU8;

typedef enum
{
   NiFpga_Main_ControlSgl_SglControl = 0x1802C,
   NiFpga_Main_ControlSgl_SglSum = 0x18028,
} NiFpga_Main_ControlSgl;

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
   NiFpga_Main_IndicatorArraySgl_SglResultArray = 0x18018,
} NiFpga_Main_IndicatorArraySgl;

typedef enum
{
   NiFpga_Main_IndicatorArraySglSize_SglResultArray = 4,
} NiFpga_Main_IndicatorArraySglSize;

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

typedef enum
{
   NiFpga_Main_ControlArraySgl_SglControlArray = 0x18020,
   NiFpga_Main_ControlArraySgl_SglSumArray = 0x1801C,
} NiFpga_Main_ControlArraySgl;

typedef enum
{
   NiFpga_Main_ControlArraySglSize_SglControlArray = 4,
   NiFpga_Main_ControlArraySglSize_SglSumArray = 4,
} NiFpga_Main_ControlArraySglSize;

typedef enum
{
   NiFpga_Main_TargetToHostFifoU16_NumbersFromFPGA = 1,
} NiFpga_Main_TargetToHostFifoU16;

typedef enum
{
   NiFpga_Main_HostToTargetFifoU32_NumbersToFPGA = 0,
} NiFpga_Main_HostToTargetFifoU32;

#if !NiFpga_VxWorks

/* Control: FxpControl */
const NiFpga_FxpTypeInfo NiFpga_Main_ControlFxp_FxpControl_TypeInfo =
{
   1,
   32,
   16
};

/* Use NiFpga_WriteU32() to access FxpControl */
const uint32_t NiFpga_Main_ControlFxp_FxpControl_Resource = 0x18044;


/* Indicator: FxpResult */
const NiFpga_FxpTypeInfo NiFpga_Main_IndicatorFxp_FxpResult_TypeInfo =
{
   1,
   33,
   17
};

/* Use NiFpga_ReadU64() to access FxpResult */
const uint32_t NiFpga_Main_IndicatorFxp_FxpResult_Resource = 0x1803C;


/* Control: FxpSum */
const NiFpga_FxpTypeInfo NiFpga_Main_ControlFxp_FxpSum_TypeInfo =
{
   1,
   32,
   16
};

/* Use NiFpga_WriteU32() to access FxpSum */
const uint32_t NiFpga_Main_ControlFxp_FxpSum_Resource = 0x18040;


/* Control: ClusterControl2 */

/* Use NiFpga_WriteArrayU8() to access ClusterControl2 */
const uint32_t NiFpga_Main_ControlClusterArray_ClusterControl2_Resource = 0x18054;
const uint32_t NiFpga_Main_ControlClusterArray_ClusterControl2_Size = 2;
const uint32_t NiFpga_Main_ControlClusterArray_ClusterControl2_PackedSizeInBytes = 8;

typedef struct NiFpga_Main_ControlClusterArray_ClusterControl2_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_ControlClusterArray_ClusterControl2_Type;

void NiFpga_Main_ControlClusterArray_ClusterControl2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlClusterArray_ClusterControl2_Type* const destination);

void NiFpga_Main_ControlClusterArray_ClusterControl2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlClusterArray_ClusterControl2_Type* const source);

/* Indicator: ClusterResult2 */

/* Use NiFpga_ReadArrayU8() to access ClusterResult2 */
const uint32_t NiFpga_Main_IndicatorClusterArray_ClusterResult2_Resource = 0x1805C;
const uint32_t NiFpga_Main_IndicatorClusterArray_ClusterResult2_Size = 2;
const uint32_t NiFpga_Main_IndicatorClusterArray_ClusterResult2_PackedSizeInBytes = 8;

typedef struct NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type;

void NiFpga_Main_IndicatorClusterArray_ClusterResult2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type* const destination);

void NiFpga_Main_IndicatorClusterArray_ClusterResult2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type* const source);

/* Control: ClusterSum2 */

/* Use NiFpga_WriteArrayU8() to access ClusterSum2 */
const uint32_t NiFpga_Main_ControlClusterArray_ClusterSum2_Resource = 0x18058;
const uint32_t NiFpga_Main_ControlClusterArray_ClusterSum2_Size = 2;
const uint32_t NiFpga_Main_ControlClusterArray_ClusterSum2_PackedSizeInBytes = 8;

typedef struct NiFpga_Main_ControlClusterArray_ClusterSum2_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_ControlClusterArray_ClusterSum2_Type;

void NiFpga_Main_ControlClusterArray_ClusterSum2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlClusterArray_ClusterSum2_Type* const destination);

void NiFpga_Main_ControlClusterArray_ClusterSum2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlClusterArray_ClusterSum2_Type* const source);

/* Control: FxpControlArray */
const NiFpga_FxpTypeInfo NiFpga_Main_ControlFxpArray_FxpControlArray_TypeInfo =
{
   1,
   32,
   16
};

/* Use NiFpga_WriteArrayU8() to access FxpControlArray */
const uint32_t NiFpga_Main_ControlFxpArray_FxpControlArray_Resource = 0x18038;
const uint32_t NiFpga_Main_ControlFxpArray_FxpControlArray_Size = 4;
const uint32_t NiFpga_Main_ControlFxpArray_FxpControlArray_PackedSizeInBytes = 16;

typedef uint32_t NiFpga_Main_ControlFxpArray_FxpControlArray_Type;

void NiFpga_Main_ControlFxpArray_FxpControlArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlFxpArray_FxpControlArray_Type* const destination);

void NiFpga_Main_ControlFxpArray_FxpControlArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlFxpArray_FxpControlArray_Type* const source);

/* Indicator: FxpResultArray */
const NiFpga_FxpTypeInfo NiFpga_Main_IndicatorFxpArray_FxpResultArray_TypeInfo =
{
   1,
   33,
   17
};

/* Use NiFpga_ReadArrayU8() to access FxpResultArray */
const uint32_t NiFpga_Main_IndicatorFxpArray_FxpResultArray_Resource = 0x18030;
const uint32_t NiFpga_Main_IndicatorFxpArray_FxpResultArray_Size = 4;
const uint32_t NiFpga_Main_IndicatorFxpArray_FxpResultArray_PackedSizeInBytes = 17;

typedef uint64_t NiFpga_Main_IndicatorFxpArray_FxpResultArray_Type;

void NiFpga_Main_IndicatorFxpArray_FxpResultArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorFxpArray_FxpResultArray_Type* const destination);

void NiFpga_Main_IndicatorFxpArray_FxpResultArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorFxpArray_FxpResultArray_Type* const source);

/* Control: FxpSumArray */
const NiFpga_FxpTypeInfo NiFpga_Main_ControlFxpArray_FxpSumArray_TypeInfo =
{
   1,
   32,
   16
};

/* Use NiFpga_WriteArrayU8() to access FxpSumArray */
const uint32_t NiFpga_Main_ControlFxpArray_FxpSumArray_Resource = 0x18034;
const uint32_t NiFpga_Main_ControlFxpArray_FxpSumArray_Size = 4;
const uint32_t NiFpga_Main_ControlFxpArray_FxpSumArray_PackedSizeInBytes = 16;

typedef uint32_t NiFpga_Main_ControlFxpArray_FxpSumArray_Type;

void NiFpga_Main_ControlFxpArray_FxpSumArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlFxpArray_FxpSumArray_Type* const destination);

void NiFpga_Main_ControlFxpArray_FxpSumArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlFxpArray_FxpSumArray_Type* const source);

/* Control: ClusterControl */
/* Use NiFpga_WriteArrayU8() to access ClusterControl */
const uint32_t NiFpga_Main_ControlCluster_ClusterControl_Resource = 0x18048;
const uint32_t NiFpga_Main_ControlCluster_ClusterControl_PackedSizeInBytes = 4;

typedef struct NiFpga_Main_ControlCluster_ClusterControl_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_ControlCluster_ClusterControl_Type;


void NiFpga_Main_ControlCluster_ClusterControl_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_ControlCluster_ClusterControl_Type* const destination);

void NiFpga_Main_ControlCluster_ClusterControl_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_ControlCluster_ClusterControl_Type* const source);

/* Indicator: ClusterResult */
/* Use NiFpga_ReadArrayU8() to access ClusterResult */
const uint32_t NiFpga_Main_IndicatorCluster_ClusterResult_Resource = 0x18050;
const uint32_t NiFpga_Main_IndicatorCluster_ClusterResult_PackedSizeInBytes = 4;

typedef struct NiFpga_Main_IndicatorCluster_ClusterResult_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_IndicatorCluster_ClusterResult_Type;


void NiFpga_Main_IndicatorCluster_ClusterResult_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorCluster_ClusterResult_Type* const destination);

void NiFpga_Main_IndicatorCluster_ClusterResult_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorCluster_ClusterResult_Type* const source);

/* Control: ClusterSum */
/* Use NiFpga_WriteArrayU8() to access ClusterSum */
const uint32_t NiFpga_Main_ControlCluster_ClusterSum_Resource = 0x1804C;
const uint32_t NiFpga_Main_ControlCluster_ClusterSum_PackedSizeInBytes = 4;

typedef struct NiFpga_Main_ControlCluster_ClusterSum_Type{
   int16_t X;
   int16_t Y;
}NiFpga_Main_ControlCluster_ClusterSum_Type;


void NiFpga_Main_ControlCluster_ClusterSum_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_ControlCluster_ClusterSum_Type* const destination);

void NiFpga_Main_ControlCluster_ClusterSum_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_ControlCluster_ClusterSum_Type* const source);

#endif /* !NiFpga_VxWorks */


#if NiFpga_Cpp
}
#endif

#endif
