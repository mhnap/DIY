#include "common.cl"

__kernel void matrix_transpose_tiled_per_column(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i_begin = (uint)get_global_id(0) * TILE_SIZE;
  const uint j = get_global_id(1);
  const uint j_local = get_local_id(1);
  const uint j_group = get_group_id(1);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  UNROLL for (uint i = 0; i < TILE_SIZE; ++i) {
#ifdef ROW_WISE
    // Read sequentially from input row
    // [j][i_begin + i]
    const uint input_index = j * COLUMN_SIZE + i_begin + i;
#else
    // Read from input column
    // [i_begin + i][j]
    const uint input_index = (i_begin + i) * COLUMN_SIZE + j;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Write transposed to tile column
    tile[i][j_local] = input[input_index];
#else
    // Write sequentially to tile row
    tile[j_local][i] = input[input_index];
#endif
  }

  barrier(CLK_LOCAL_MEM_FENCE);

  UNROLL for (uint i = 0; i < TILE_SIZE; ++i) {
#ifdef ROW_WISE
    // Write sequentially to output row
    // [i_begin + j_local][j_group * TILE_SIZE + i]
    const uint output_index = (i_begin + j_local) * COLUMN_SIZE + j_group * TILE_SIZE + i;
#else
    // Write to output column
    // [j_group * TILE_SIZE + i][i_begin + j_local]
    const uint output_index = (j_group * TILE_SIZE + i) * COLUMN_SIZE + i_begin + j_local;
#endif
#ifdef TRANSPOSE_ON_TILE_WRITE
    // Read sequentially from tile row
    output[output_index] = tile[j_local][i];
#else
    // Read transposed from tile column
    output[output_index] = tile[i][j_local];
#endif
  }
}
