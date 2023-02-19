__kernel void matrix_transpose_tiled_per_elem(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j = get_global_id(1);
  const uint i_local = get_local_id(0);
  const uint j_local = get_local_id(1);
  const uint i_group = get_group_id(0);
  const uint j_group = get_group_id(1);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

#ifdef ROW_WISE
  // Read from input row
  // [i][j]
  const uint input_index = i * COLUMN_SIZE + j;
#else
  // Read from input column
  // [j][i]
  const uint input_index = j * COLUMN_SIZE + i;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
  // Write transposed to tile column
  tile[j_local][i_local] = input[input_index];
#else
  // Write sequentially to tile row
  tile[i_local][j_local] = input[input_index];
#endif

  barrier(CLK_LOCAL_MEM_FENCE);

#ifdef ROW_WISE
  // Write to output row
  // [j_group * TILE_SIZE + i_local][i_group * TILE_SIZE + j_local]
  const uint output_index = (j_group * TILE_SIZE + i_local) * COLUMN_SIZE + i_group * TILE_SIZE + j_local;
#else
  // Write to output column
  // [i_group * TILE_SIZE + j_local][j_group * TILE_SIZE + i_local]
  const uint output_index = (i_group * TILE_SIZE + j_local) * COLUMN_SIZE + j_group * TILE_SIZE + i_local;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
  // Read sequentially from tile row
  output[output_index] = tile[i_local][j_local];
#else
  // Read transposed from tile column
  output[output_index] = tile[j_local][i_local];
#endif
}
