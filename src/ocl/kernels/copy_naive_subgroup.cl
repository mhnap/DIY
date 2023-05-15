#include "common.cl"

#ifdef SUB_GROUP_SIZE
REQD_SUB_GROUP_SIZE(SUB_GROUP_SIZE)
#endif

__kernel void copy_naive_subgroup(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint globalId = get_global_id(0);

  // Read data
  const DATA_TYPE val = intel_sub_group_block_read(input + globalId - get_sub_group_local_id());

  // Write data
  intel_sub_group_block_write(output + globalId - get_sub_group_local_id(), val);
}
