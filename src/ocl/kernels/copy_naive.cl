__kernel void copy_naive(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  size_t globalId = get_global_id(0);
  output[globalId] = input[globalId];
}
