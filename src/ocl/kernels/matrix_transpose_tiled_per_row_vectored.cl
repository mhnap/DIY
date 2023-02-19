#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void matrix_transpose_tiled_per_row_vectored(const __global DATA_TYPE* input,
                                                      __global DATA_TYPE* output) {
  const uint i = get_global_id(0);
  const uint j_begin = (uint)get_global_id(1) * TILE_SIZE;
  const uint i_local = get_local_id(0);
  const uint group_id = get_group_id(0);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  // Read vectored from input row
  const uint input_index = i * COLUMN_SIZE + j_begin;
  const VEC_TYPE input_vec = VLOAD(0, input + input_index);

#ifdef TRANSPOSE_ON_TILE_WRITE
  // Write transposed to tile column
  UNROLL for (uint j = 0; j < TILE_SIZE; ++j) { tile[j][i_local] = input_vec[j]; }
#else
  // Write vectored to tile row
  VSTORE(input_vec, 0, tile + i_local);
#endif

  barrier(CLK_LOCAL_MEM_FENCE);

#ifdef TRANSPOSE_ON_TILE_WRITE
  // Read vectored from tile row
  const VEC_TYPE tile_vec = VLOAD(0, tile[i_local]);
#else
  // Read transposed from tile column
  VEC_TYPE tile_vec;
  UNROLL for (uint j = 0; j < TILE_SIZE; ++j) { tile_vec[j] = tile[j][i_local]; }
#endif

  // Write vectored to output row
  const uint output_index = (j_begin + i_local) * COLUMN_SIZE + group_id * TILE_SIZE;
  VSTORE(tile_vec, 0, output + output_index);
}
