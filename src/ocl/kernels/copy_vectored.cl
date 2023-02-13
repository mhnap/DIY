#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void copy_vectored(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  size_t globalId = get_global_id(0);
  size_t index = globalId * VEC_SIZE;

#ifndef REMAINDER_ITEM
  // Size is divided by vec_size without remainder
  VEC_TYPE v = VLOAD(0, input + index);
  VSTORE(v, 0, output + index);
#else
  if (globalId < REMAINDER_ITEM) {
    // Can read vectored
    VEC_TYPE v = VLOAD(0, input + index);
    VSTORE(v, 0, output + index);
  } else if (globalId == REMAINDER_ITEM) {
    // Need to handle remainders, read one by one
    for (uint i = 0; i < REMAINDER_SIZE; ++i) {
      output[index + i] = input[index + i];
    }
  } else {
    // Should never go here as globalId cannot be greater than REMAINDER_ITEM
    printf("LOGIC ERROR!\n");
  }
#endif
}
