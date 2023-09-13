__kernel void matrix_rotate(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint x = get_global_id(0);
  const uint y = get_global_id(1);
  // output[y][END - x] = input[x][y]
  output[y * SIZE + SIZE - 1 - x] = input[x * SIZE + y];
}
