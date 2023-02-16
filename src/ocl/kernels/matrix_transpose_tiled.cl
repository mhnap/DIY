__kernel void matrix_transpose_tiled(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j_begin = get_global_id(1) * TILE_SIZE;
  const uint i_local = get_local_id(0);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  for (uint j = 0; j < TILE_SIZE; ++j) {
    // [i][j_begin + j]
    const uint input_index = i * COLUMN_SIZE + j_begin + j;
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Read sequentially from input row and write transposed to tile column
    tile[j][i_local] = input[input_index];
#else
    // Read sequentially from input row and write sequentially to tile row
    tile[i_local][j] = input[input_index];
#endif
  }

  barrier(CLK_LOCAL_MEM_FENCE);

  for (uint j = 0; j < TILE_SIZE; ++j) {
    // [j_begin + i_local][get_group_id(0) * TILE_SIZE + j]
    const uint output_index = (j_begin + i_local) * COLUMN_SIZE + get_group_id(0) * TILE_SIZE + j;
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Read sequentially from tile row and write sequentially to output row
    output[output_index] = tile[i_local][j];
#else
    // Read transposed from tile column and write sequentially to output row
    output[output_index] = tile[j][i_local];
#endif
  }
}
