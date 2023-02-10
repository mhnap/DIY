#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void copy_vectored(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  size_t globalId = get_global_id(0);
  size_t index = globalId * VEC_SIZE;

  VEC_TYPE v = VLOAD(0, input + index);
  VSTORE(v, 0, output + index);

  // TODO: Handle remainder
}
