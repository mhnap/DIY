#define _CAT(a, b) a##b
#define CAT(a, b) _CAT(a, b)

#define _STRINGIFY(s) #s
#define STRINGIFY(s) _STRINGIFY(s)

#define UNROLL __attribute__((opencl_unroll_hint))

void common_print_info() {
  const uint workDim = get_work_dim();
  if (workDim == 1) {
    printf("0:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d]\n",
           get_global_id(0), get_local_id(0), get_group_id(0), get_global_size(0), get_local_size(0),
           get_num_groups(0));
  } else if (workDim == 2) {
    printf("0:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d] "
           "1:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d]\n",
           get_global_id(0), get_local_id(0), get_group_id(0), get_global_size(0), get_local_size(0),
           get_num_groups(0), get_global_id(1), get_local_id(1), get_group_id(1), get_global_size(1),
           get_local_size(1), get_num_groups(1));
  } else {
    printf("0:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d] "
           "1:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d] "
           "2:[globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d]\n",
           get_global_id(0), get_local_id(0), get_group_id(0), get_global_size(0), get_local_size(0),
           get_num_groups(0), get_global_id(1), get_local_id(1), get_group_id(1), get_global_size(1),
           get_local_size(1), get_num_groups(1), get_global_id(2), get_local_id(2), get_group_id(2),
           get_global_size(2), get_local_size(2), get_num_groups(2));
  }
}
