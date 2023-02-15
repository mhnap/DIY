#include "ocl/core/engine.hpp"
#include <iostream>

int main() {
  ocl::Engine engine("print_info", {4, 8, 3});
  engine.setLocalWorkSizes({4, 4, 1});
  engine.enableProfiling();
  engine.run();
  std::cout << "Execution time: " << engine.getExecutionTime();
}
