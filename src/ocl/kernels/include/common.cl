// globalId = groupId * localSize + localId
// globalSize % localSize == 0
// numGroups = globalSize / localSize

// localId = subGroupId * subGroupSize + subGroupLocalId
// numSubGroups = ceil(localSize / subGroupSize)

// localSize SHOULD BE >= maxSubGroupSize, because sub_group_block_read and sub_group_block_write
// values are p[ sub_group_local_id + max_sub_group_size ]

// If using sub_group_block_read or sub_group_block_write, always should set REQD_SUB_GROUP_SIZE
// to not have the problem described above

#define _CAT(a, b) a##b
#define CAT(a, b) _CAT(a, b)

#define _STRINGIFY(s) #s
#define STRINGIFY(s) _STRINGIFY(s)

#define UNROLL __attribute__((opencl_unroll_hint))

#ifdef cl_intel_required_subgroup_size
#define REQD_SUB_GROUP_SIZE(sg_size) __attribute__((intel_reqd_sub_group_size(sg_size)))
#else
#define REQD_SUB_GROUP_SIZE(sg_size)
#endif

void common_print_info(bool print_subgroup_info) {
  const uint workDim = get_work_dim();
  if (workDim == 1) {
    if (get_global_id(0) == 0) {
      if (print_subgroup_info) {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d]"
               "  SG SIZES "
               "subGroupSize=%d;maxSubGroupSize=%d;numSubGroups=%d\n",
               get_global_size(0), get_local_size(0), get_num_groups(0), get_sub_group_size(),
               get_max_sub_group_size(), get_num_sub_groups());
      } else {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d]\n",
               get_global_size(0), get_local_size(0), get_num_groups(0));
      }
    }
    if (print_subgroup_info) {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;]"
             "  SG IDS "
             "subGroupId=%d;subGroupLocalId=%d\n",
             get_global_id(0), get_local_id(0), get_group_id(0), get_sub_group_id(),
             get_sub_group_local_id());
    } else {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;]\n",
             get_global_id(0), get_local_id(0), get_group_id(0));
    }
  } else if (workDim == 2) {
    if (get_global_id(0) == 0 && get_global_id(1) == 0) {
      if (print_subgroup_info) {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "1d:[globalSize=%d;localSize=%d;numGroups=%d]"
               "  SG SIZES "
               "subGroupSize=%d;maxSubGroupSize=%d;numSubGroups=%d\n",
               get_global_size(0), get_local_size(0), get_num_groups(0), get_global_size(1),
               get_local_size(1), get_num_groups(1), get_sub_group_size(), get_max_sub_group_size(),
               get_num_sub_groups());
      } else {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "1d:[globalSize=%d;localSize=%d;numGroups=%d]\n",
               get_global_size(0), get_local_size(0), get_num_groups(0), get_global_size(1),
               get_local_size(1), get_num_groups(1));
      }
    }
    if (print_subgroup_info) {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;] "
             "1d:[globalId=%d;localId=%d;groupId=%d;]"
             "  SG IDS "
             "subGroupId=%d;subGroupLocalId=%d\n",
             get_global_id(0), get_local_id(0), get_group_id(0), get_global_id(1), get_local_id(1),
             get_group_id(1), get_sub_group_id(), get_sub_group_local_id());
    } else {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;] "
             "1d:[globalId=%d;localId=%d;groupId=%d;]\n",
             get_global_id(0), get_local_id(0), get_group_id(0), get_global_id(1), get_local_id(1),
             get_group_id(1));
    }
  } else {
    if (get_global_id(0) == 0 && get_global_id(1) == 0 && get_global_id(2) == 0) {
      if (print_subgroup_info) {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "1d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "2d:[globalSize=%d;localSize=%d;numGroups=%d]"
               "  SG SIZES "
               "subGroupSize=%d;maxSubGroupSize=%d;numSubGroups=%d\n",
               get_global_size(0), get_local_size(0), get_num_groups(0), get_global_size(1),
               get_local_size(1), get_num_groups(1), get_global_size(2), get_local_size(2), get_num_groups(2),
               get_sub_group_size(), get_max_sub_group_size(), get_num_sub_groups());
      } else {
        printf("SIZES "
               "0d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "1d:[globalSize=%d;localSize=%d;numGroups=%d] "
               "2d:[globalSize=%d;localSize=%d;numGroups=%d]\n",
               get_global_size(0), get_local_size(0), get_num_groups(0), get_global_size(1),
               get_local_size(1), get_num_groups(1), get_global_size(2), get_local_size(2),
               get_num_groups(2));
      }
    }
    if (print_subgroup_info) {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;] "
             "1d:[globalId=%d;localId=%d;groupId=%d;] "
             "2d:[globalId=%d;localId=%d;groupId=%d;]"
             "  SG IDS "
             "subGroupId=%d;subGroupLocalId=%d\n",
             get_global_id(0), get_local_id(0), get_group_id(0), get_global_id(1), get_local_id(1),
             get_group_id(1), get_global_id(2), get_local_id(2), get_group_id(2), get_sub_group_id(),
             get_sub_group_local_id());
    } else {
      printf("IDS "
             "0d:[globalId=%d;localId=%d;groupId=%d;] "
             "1d:[globalId=%d;localId=%d;groupId=%d;] "
             "2d:[globalId=%d;localId=%d;groupId=%d;]\n",
             get_global_id(0), get_local_id(0), get_group_id(0), get_global_id(1), get_local_id(1),
             get_group_id(1), get_global_id(2), get_local_id(2), get_group_id(2));
    }
  }
}
