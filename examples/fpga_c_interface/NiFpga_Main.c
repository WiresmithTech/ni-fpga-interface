#include "NiFpga_Main.h"

#if !NiFpga_VxWorks




void NiFpga_Main_ControlClusterArray_ClusterControl2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlClusterArray_ClusterControl2_Type* const destination)
{
   destination[0].X = 0;
   destination[0].X |= (packedData[0] & 0xFF) << 8;
   destination[0].X |= (packedData[1] & 0xFF);
   destination[0].Y = 0;
   destination[0].Y |= (packedData[2] & 0xFF) << 8;
   destination[0].Y |= (packedData[3] & 0xFF);
   destination[1].X = 0;
   destination[1].X |= (packedData[4] & 0xFF) << 8;
   destination[1].X |= (packedData[5] & 0xFF);
   destination[1].Y = 0;
   destination[1].Y |= (packedData[6] & 0xFF) << 8;
   destination[1].Y |= (packedData[7] & 0xFF);
}

void NiFpga_Main_ControlClusterArray_ClusterControl2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlClusterArray_ClusterControl2_Type* const source)
{
   packedData[0] = (uint8_t)((source[0].X >> 8) & 0xFF);
   packedData[1] = (uint8_t)(source[0].X & 0xFF);
   packedData[2] = (uint8_t)((source[0].Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)(source[0].Y & 0xFF);
   packedData[4] = (uint8_t)((source[1].X >> 8) & 0xFF);
   packedData[5] = (uint8_t)(source[1].X & 0xFF);
   packedData[6] = (uint8_t)((source[1].Y >> 8) & 0xFF);
   packedData[7] = (uint8_t)(source[1].Y & 0xFF);
}

void NiFpga_Main_IndicatorClusterArray_ClusterResult2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type* const destination)
{
   destination[0].X = 0;
   destination[0].X |= (packedData[0] & 0xFF) << 8;
   destination[0].X |= (packedData[1] & 0xFF);
   destination[0].Y = 0;
   destination[0].Y |= (packedData[2] & 0xFF) << 8;
   destination[0].Y |= (packedData[3] & 0xFF);
   destination[1].X = 0;
   destination[1].X |= (packedData[4] & 0xFF) << 8;
   destination[1].X |= (packedData[5] & 0xFF);
   destination[1].Y = 0;
   destination[1].Y |= (packedData[6] & 0xFF) << 8;
   destination[1].Y |= (packedData[7] & 0xFF);
}

void NiFpga_Main_IndicatorClusterArray_ClusterResult2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorClusterArray_ClusterResult2_Type* const source)
{
   packedData[0] = (uint8_t)((source[0].X >> 8) & 0xFF);
   packedData[1] = (uint8_t)(source[0].X & 0xFF);
   packedData[2] = (uint8_t)((source[0].Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)(source[0].Y & 0xFF);
   packedData[4] = (uint8_t)((source[1].X >> 8) & 0xFF);
   packedData[5] = (uint8_t)(source[1].X & 0xFF);
   packedData[6] = (uint8_t)((source[1].Y >> 8) & 0xFF);
   packedData[7] = (uint8_t)(source[1].Y & 0xFF);
}

void NiFpga_Main_ControlClusterArray_ClusterSum2_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlClusterArray_ClusterSum2_Type* const destination)
{
   destination[0].X = 0;
   destination[0].X |= (packedData[0] & 0xFF) << 8;
   destination[0].X |= (packedData[1] & 0xFF);
   destination[0].Y = 0;
   destination[0].Y |= (packedData[2] & 0xFF) << 8;
   destination[0].Y |= (packedData[3] & 0xFF);
   destination[1].X = 0;
   destination[1].X |= (packedData[4] & 0xFF) << 8;
   destination[1].X |= (packedData[5] & 0xFF);
   destination[1].Y = 0;
   destination[1].Y |= (packedData[6] & 0xFF) << 8;
   destination[1].Y |= (packedData[7] & 0xFF);
}

void NiFpga_Main_ControlClusterArray_ClusterSum2_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlClusterArray_ClusterSum2_Type* const source)
{
   packedData[0] = (uint8_t)((source[0].X >> 8) & 0xFF);
   packedData[1] = (uint8_t)(source[0].X & 0xFF);
   packedData[2] = (uint8_t)((source[0].Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)(source[0].Y & 0xFF);
   packedData[4] = (uint8_t)((source[1].X >> 8) & 0xFF);
   packedData[5] = (uint8_t)(source[1].X & 0xFF);
   packedData[6] = (uint8_t)((source[1].Y >> 8) & 0xFF);
   packedData[7] = (uint8_t)(source[1].Y & 0xFF);
}

void NiFpga_Main_ControlFxpArray_FxpControlArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlFxpArray_FxpControlArray_Type* const destination)
{
   destination[0] = 0;
   destination[0] |= (packedData[0] & 0xFFULL) << 24;
   destination[0] |= (packedData[1] & 0xFF) << 16;
   destination[0] |= (packedData[2] & 0xFF) << 8;
   destination[0] |= (packedData[3] & 0xFF);
   destination[1] = 0;
   destination[1] |= (packedData[4] & 0xFFULL) << 24;
   destination[1] |= (packedData[5] & 0xFF) << 16;
   destination[1] |= (packedData[6] & 0xFF) << 8;
   destination[1] |= (packedData[7] & 0xFF);
   destination[2] = 0;
   destination[2] |= (packedData[8] & 0xFFULL) << 24;
   destination[2] |= (packedData[9] & 0xFF) << 16;
   destination[2] |= (packedData[10] & 0xFF) << 8;
   destination[2] |= (packedData[11] & 0xFF);
   destination[3] = 0;
   destination[3] |= (packedData[12] & 0xFFULL) << 24;
   destination[3] |= (packedData[13] & 0xFF) << 16;
   destination[3] |= (packedData[14] & 0xFF) << 8;
   destination[3] |= (packedData[15] & 0xFF);
}

void NiFpga_Main_ControlFxpArray_FxpControlArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlFxpArray_FxpControlArray_Type* const source)
{
   packedData[0] = (uint8_t)((source[0] >> 24) & 0xFF);
   packedData[1] = (uint8_t)((source[0] >> 16) & 0xFF);
   packedData[2] = (uint8_t)((source[0] >> 8) & 0xFF);
   packedData[3] = (uint8_t)(source[0] & 0xFF);
   packedData[4] = (uint8_t)((source[1] >> 24) & 0xFF);
   packedData[5] = (uint8_t)((source[1] >> 16) & 0xFF);
   packedData[6] = (uint8_t)((source[1] >> 8) & 0xFF);
   packedData[7] = (uint8_t)(source[1] & 0xFF);
   packedData[8] = (uint8_t)((source[2] >> 24) & 0xFF);
   packedData[9] = (uint8_t)((source[2] >> 16) & 0xFF);
   packedData[10] = (uint8_t)((source[2] >> 8) & 0xFF);
   packedData[11] = (uint8_t)(source[2] & 0xFF);
   packedData[12] = (uint8_t)((source[3] >> 24) & 0xFF);
   packedData[13] = (uint8_t)((source[3] >> 16) & 0xFF);
   packedData[14] = (uint8_t)((source[3] >> 8) & 0xFF);
   packedData[15] = (uint8_t)(source[3] & 0xFF);
}

void NiFpga_Main_IndicatorFxpArray_FxpResultArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorFxpArray_FxpResultArray_Type* const destination)
{
   destination[0] = 0;
   destination[0] |= (packedData[0] & 0xFFULL) << 25;
   destination[0] |= (packedData[1] & 0xFF) << 17;
   destination[0] |= (packedData[2] & 0xFF) << 9;
   destination[0] |= (packedData[3] & 0xFF) << 1;
   destination[0] |= ((packedData[4] >> 7) & 0x1);
   destination[1] = 0;
   destination[1] |= (packedData[4] & 0x7FULL) << 26;
   destination[1] |= (packedData[5] & 0xFF) << 18;
   destination[1] |= (packedData[6] & 0xFF) << 10;
   destination[1] |= (packedData[7] & 0xFF) << 2;
   destination[1] |= ((packedData[8] >> 6) & 0x3);
   destination[2] = 0;
   destination[2] |= (packedData[8] & 0x3FULL) << 27;
   destination[2] |= (packedData[9] & 0xFF) << 19;
   destination[2] |= (packedData[10] & 0xFF) << 11;
   destination[2] |= (packedData[11] & 0xFF) << 3;
   destination[2] |= ((packedData[12] >> 5) & 0x7);
   destination[3] = 0;
   destination[3] |= (packedData[12] & 0x1FULL) << 28;
   destination[3] |= (packedData[13] & 0xFF) << 20;
   destination[3] |= (packedData[14] & 0xFF) << 12;
   destination[3] |= (packedData[15] & 0xFF) << 4;
   destination[3] |= ((packedData[16] >> 4) & 0xF);
}

void NiFpga_Main_IndicatorFxpArray_FxpResultArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorFxpArray_FxpResultArray_Type* const source)
{
   packedData[0] = (uint8_t)((source[0] >> 25) & 0xFF);
   packedData[1] = (uint8_t)((source[0] >> 17) & 0xFF);
   packedData[2] = (uint8_t)((source[0] >> 9) & 0xFF);
   packedData[3] = (uint8_t)((source[0] >> 1) & 0xFF);
   packedData[4] = (uint8_t)((source[0] & 0x1) << 7);
   packedData[4] |= (uint8_t)((source[1] >> 26) & 0x7F);
   packedData[5] = (uint8_t)((source[1] >> 18) & 0xFF);
   packedData[6] = (uint8_t)((source[1] >> 10) & 0xFF);
   packedData[7] = (uint8_t)((source[1] >> 2) & 0xFF);
   packedData[8] = (uint8_t)((source[1] & 0x3) << 6);
   packedData[8] |= (uint8_t)((source[2] >> 27) & 0x3F);
   packedData[9] = (uint8_t)((source[2] >> 19) & 0xFF);
   packedData[10] = (uint8_t)((source[2] >> 11) & 0xFF);
   packedData[11] = (uint8_t)((source[2] >> 3) & 0xFF);
   packedData[12] = (uint8_t)((source[2] & 0x7) << 5);
   packedData[12] |= (uint8_t)((source[3] >> 28) & 0x1F);
   packedData[13] = (uint8_t)((source[3] >> 20) & 0xFF);
   packedData[14] = (uint8_t)((source[3] >> 12) & 0xFF);
   packedData[15] = (uint8_t)((source[3] >> 4) & 0xFF);
   packedData[16] = (uint8_t)((source[3] & 0xF) << 4);
}

void NiFpga_Main_ControlFxpArray_FxpSumArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Main_ControlFxpArray_FxpSumArray_Type* const destination)
{
   destination[0] = 0;
   destination[0] |= (packedData[0] & 0xFFULL) << 24;
   destination[0] |= (packedData[1] & 0xFF) << 16;
   destination[0] |= (packedData[2] & 0xFF) << 8;
   destination[0] |= (packedData[3] & 0xFF);
   destination[1] = 0;
   destination[1] |= (packedData[4] & 0xFFULL) << 24;
   destination[1] |= (packedData[5] & 0xFF) << 16;
   destination[1] |= (packedData[6] & 0xFF) << 8;
   destination[1] |= (packedData[7] & 0xFF);
   destination[2] = 0;
   destination[2] |= (packedData[8] & 0xFFULL) << 24;
   destination[2] |= (packedData[9] & 0xFF) << 16;
   destination[2] |= (packedData[10] & 0xFF) << 8;
   destination[2] |= (packedData[11] & 0xFF);
   destination[3] = 0;
   destination[3] |= (packedData[12] & 0xFFULL) << 24;
   destination[3] |= (packedData[13] & 0xFF) << 16;
   destination[3] |= (packedData[14] & 0xFF) << 8;
   destination[3] |= (packedData[15] & 0xFF);
}

void NiFpga_Main_ControlFxpArray_FxpSumArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Main_ControlFxpArray_FxpSumArray_Type* const source)
{
   packedData[0] = (uint8_t)((source[0] >> 24) & 0xFF);
   packedData[1] = (uint8_t)((source[0] >> 16) & 0xFF);
   packedData[2] = (uint8_t)((source[0] >> 8) & 0xFF);
   packedData[3] = (uint8_t)(source[0] & 0xFF);
   packedData[4] = (uint8_t)((source[1] >> 24) & 0xFF);
   packedData[5] = (uint8_t)((source[1] >> 16) & 0xFF);
   packedData[6] = (uint8_t)((source[1] >> 8) & 0xFF);
   packedData[7] = (uint8_t)(source[1] & 0xFF);
   packedData[8] = (uint8_t)((source[2] >> 24) & 0xFF);
   packedData[9] = (uint8_t)((source[2] >> 16) & 0xFF);
   packedData[10] = (uint8_t)((source[2] >> 8) & 0xFF);
   packedData[11] = (uint8_t)(source[2] & 0xFF);
   packedData[12] = (uint8_t)((source[3] >> 24) & 0xFF);
   packedData[13] = (uint8_t)((source[3] >> 16) & 0xFF);
   packedData[14] = (uint8_t)((source[3] >> 8) & 0xFF);
   packedData[15] = (uint8_t)(source[3] & 0xFF);
}

void NiFpga_Main_ControlCluster_ClusterControl_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_ControlCluster_ClusterControl_Type* const destination)
{
   (*destination).X = 0;
   (*destination).X |= (packedData[0] & 0xFF) << 8;
   (*destination).X |= (packedData[1] & 0xFF);
   (*destination).Y = 0;
   (*destination).Y |= (packedData[2] & 0xFF) << 8;
   (*destination).Y |= (packedData[3] & 0xFF);
}

void NiFpga_Main_ControlCluster_ClusterControl_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_ControlCluster_ClusterControl_Type* const source)
{
   packedData[0] = (uint8_t)(((*source).X >> 8) & 0xFF);
   packedData[1] = (uint8_t)((*source).X & 0xFF);
   packedData[2] = (uint8_t)(((*source).Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)((*source).Y & 0xFF);
}

void NiFpga_Main_IndicatorCluster_ClusterResult_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_IndicatorCluster_ClusterResult_Type* const destination)
{
   (*destination).X = 0;
   (*destination).X |= (packedData[0] & 0xFF) << 8;
   (*destination).X |= (packedData[1] & 0xFF);
   (*destination).Y = 0;
   (*destination).Y |= (packedData[2] & 0xFF) << 8;
   (*destination).Y |= (packedData[3] & 0xFF);
}

void NiFpga_Main_IndicatorCluster_ClusterResult_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_IndicatorCluster_ClusterResult_Type* const source)
{
   packedData[0] = (uint8_t)(((*source).X >> 8) & 0xFF);
   packedData[1] = (uint8_t)((*source).X & 0xFF);
   packedData[2] = (uint8_t)(((*source).Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)((*source).Y & 0xFF);
}

void NiFpga_Main_ControlCluster_ClusterSum_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Main_ControlCluster_ClusterSum_Type* const destination)
{
   (*destination).X = 0;
   (*destination).X |= (packedData[0] & 0xFF) << 8;
   (*destination).X |= (packedData[1] & 0xFF);
   (*destination).Y = 0;
   (*destination).Y |= (packedData[2] & 0xFF) << 8;
   (*destination).Y |= (packedData[3] & 0xFF);
}

void NiFpga_Main_ControlCluster_ClusterSum_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Main_ControlCluster_ClusterSum_Type* const source)
{
   packedData[0] = (uint8_t)(((*source).X >> 8) & 0xFF);
   packedData[1] = (uint8_t)((*source).X & 0xFF);
   packedData[2] = (uint8_t)(((*source).Y >> 8) & 0xFF);
   packedData[3] = (uint8_t)((*source).Y & 0xFF);
}

#endif /* !NiFpga_VxWorks */
