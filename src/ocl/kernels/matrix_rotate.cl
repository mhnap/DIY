__kernel void matrix_rotate(__global DATA_TYPE* input, __global DATA_TYPE* output)
{
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  // output[y][END - x] = input[x][y]
  output[y * SIZE + SIZE - 1 - x] = input[x * SIZE + y];
}