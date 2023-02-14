#include "ocl/core/engine.hpp"
#include <iostream>

int main() {
  ocl::Engine engine("print_info", {1024});
  //  engine.setLocalWorkSizes({513}); Error -54: CL_INVALID_WORK_GROUP_SIZE
  engine.setLocalWorkSizes({512});
  engine.enableProfiling();
  engine.run();
  std::cout << "Execution time: " << engine.getExecutionTime();
}
