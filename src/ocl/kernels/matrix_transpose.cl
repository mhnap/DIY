__kernel void matrix_transpose(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  size_t i = get_global_id(0);
  size_t j = get_global_id(1);
  // output[j][i] = input[i][j]
  output[j * ROW_SIZE + i] = input[i * COLUMN_SIZE + j];
}
