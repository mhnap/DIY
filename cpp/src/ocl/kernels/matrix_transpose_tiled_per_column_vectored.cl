#include "common.cl"

#define VEC_TYPE CAT(DATA_TYPE, VEC_SIZE)
#define VLOAD CAT(vload, VEC_SIZE)
#define VSTORE CAT(vstore, VEC_SIZE)

__kernel void matrix_transpose_tiled_per_column_vectored(const __global DATA_TYPE* input,
                                                         __global DATA_TYPE* output) {
  const uint i_begin = (uint)get_global_id(0) * TILE_SIZE;
  const uint j = get_global_id(1);
  const uint j_local = get_local_id(1);
  const uint j_group = get_group_id(1);

  __local DATA_TYPE tile[TILE_SIZE][TILE_SIZE];

  // Read vectored from input row
  const uint input_index = j * COLUMN_SIZE + i_begin;
  const VEC_TYPE input_vec = VLOAD(0, input + input_index);

#ifdef TRANSPOSE_ON_TILE_WRITE
  // Write transposed to tile column
  UNROLL for (uint i = 0; i < TILE_SIZE; ++i) { tile[i][j_local] = input_vec[i]; }
#else
  // Write vectored to tile row
  VSTORE(input_vec, 0, tile + j_local);
#endif

  barrier(CLK_LOCAL_MEM_FENCE);

#ifdef TRANSPOSE_ON_TILE_WRITE
  // Read vectored from tile row
  const VEC_TYPE tile_vec = VLOAD(0, tile[j_local]);
#else
  // Read transposed from tile column
  VEC_TYPE tile_vec;
  UNROLL for (uint i = 0; i < TILE_SIZE; ++i) { tile_vec[i] = tile[i][j_local]; }
#endif

  // Write vectored to output row
  const uint output_index = (i_begin + j_local) * COLUMN_SIZE + j_group * TILE_SIZE;
  VSTORE(tile_vec, 0, output + output_index);
}
