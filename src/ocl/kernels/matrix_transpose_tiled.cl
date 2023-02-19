#include "common.cl"

__kernel void matrix_transpose_tiled(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j_begin = (uint)get_global_id(1) * TILE_SIZE;
  const uint i_local = get_local_id(0);
  const uint group_id = get_group_id(0);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  UNROLL for (uint j = 0; j < TILE_SIZE; ++j) {
#ifdef ROW_WISE
    // Read sequentially from input row
    // [i][j_begin + j]
    const uint input_index = i * COLUMN_SIZE + j_begin + j;
#else
    // Read from input column
    // [j_begin + j][i]
    const uint input_index = (j_begin + j) * COLUMN_SIZE + i;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Write transposed to tile column
    tile[j][i_local] = input[input_index];
#else
    // Write sequentially to tile row
    tile[i_local][j] = input[input_index];
#endif
  }

  barrier(CLK_LOCAL_MEM_FENCE);

  UNROLL for (uint j = 0; j < TILE_SIZE; ++j) {
#ifdef ROW_WISE
    // Write sequentially to output row
    // [j_begin + i_local][get_group_id(0) * TILE_SIZE + j]
    const uint output_index = (j_begin + i_local) * COLUMN_SIZE + group_id * TILE_SIZE + j;
#else
    // Write to output column
    // [get_group_id(0) * TILE_SIZE + j][j_begin + i_local]
    const uint output_index = (group_id * TILE_SIZE + j) * COLUMN_SIZE + j_begin + i_local;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Read sequentially from tile row
    output[output_index] = tile[i_local][j];
#else
    // Read transposed from tile column
    output[output_index] = tile[j][i_local];
#endif
  }
}
