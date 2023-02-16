#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void matrix_transpose_tiled_vectored(const __global DATA_TYPE* input, __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j_begin = (uint)get_global_id(1) * TILE_SIZE;
  const uint i_local = get_local_id(0);
  const uint group_id = get_group_id(0);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  // Read vectored from input row
  const uint input_index = i * COLUMN_SIZE + j_begin;
  const VEC_TYPE input_data = VLOAD(0, input + input_index);

  // Write transposed to tile column
  UNROLL for (uint j = 0; j < TILE_SIZE; ++j) { tile[j][i_local] = input_data[j]; }

  barrier(CLK_LOCAL_MEM_FENCE);

  // Read vectored from tile row and write vectored to output row
  const uint output_index = (j_begin + i_local) * COLUMN_SIZE + group_id * TILE_SIZE;
  VSTORE(VLOAD(0, tile[i_local]), 0, output + output_index);
}
