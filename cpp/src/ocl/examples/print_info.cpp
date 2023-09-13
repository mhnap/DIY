#include "ocl/core/engine.hpp"
#include <iostream>

int main() {
  ocl::Engine engine("print_info", {16, 10, 15});
  engine.setLocalWorkSizes({8, 10, 5});
  engine.addCompilerOptionDefine("PRINT_SUBGROUP_INFO", true);
  engine.addCompilerOptionDefine("SUB_GROUP_SIZE", 8);
  engine.enableProfiling();
  engine.run();
  std::cout << "Execution time: " << engine.getExecutionTime();
}
