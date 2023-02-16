__kernel void matrix_transpose_naive(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j = get_global_id(1);
  // output[j][i] = input[i][j]
  output[j * ROW_SIZE + i] = input[i * COLUMN_SIZE + j];
}
