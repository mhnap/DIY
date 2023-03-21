// globalId = groupId * localSize + localId
// globalSize % localSize == 0
// numGroups = globalSize / localSize

#include "common.cl"

__kernel void print_info() { common_print_info(); }
