__kernel void print_info()
{
  int globalId = get_global_id(0);
  int localId = get_local_id(0);
  int groupId = get_group_id(0);
  int globalSize = get_global_size(0);
  int localSize = get_local_size(0);
  int numGroups = get_num_groups(0);
  int workDim = get_work_dim();
  // globalId = groupId * localSize + localId
  // globalSize % localSize == 0
  // numGroups = globalSize / localSize
  printf("globalId=%d;localId=%d;groupId=%d;globalSize=%d;localSize=%d;numGroups=%d;workDim=%d;\n",
    globalId, localId, groupId, globalSize, localSize, numGroups, workDim);
 }