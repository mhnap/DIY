#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void copy_vectored(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint globalId = get_global_id(0);
  const uint index = globalId * VEC_SIZE;

#ifndef REMAINDER_SIZE
  // Size is divided by vec_size without remainder
  VEC_TYPE v = VLOAD(0, input + index);
  VSTORE(v, 0, output + index);
#else
  const uint globalSize = get_global_size(0);
  if (globalId < globalSize - 1) {
    // Can read vectored
    VEC_TYPE v = VLOAD(0, input + index);
    VSTORE(v, 0, output + index);
  } else { // globalId == globalSize - 1
    // Need to handle remainder, read one by one
    for (uint i = 0; i < REMAINDER_SIZE; ++i) {
      output[index + i] = input[index + i];
    }
  }
#endif
}
