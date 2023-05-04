// globalId = groupId * localSize + localId
// globalSize % localSize == 0
// numGroups = globalSize / localSize

#include "common.cl"

#ifndef PRINT_SUBGROUP_INFO
#define PRINT_SUBGROUP_INFO false
#endif

#ifdef SUB_GROUP_SIZE
REQD_SUB_GROUP_SIZE(SUB_GROUP_SIZE)
#endif

__kernel void print_info() { common_print_info(PRINT_SUBGROUP_INFO); }
