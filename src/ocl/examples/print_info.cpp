#include "ocl/core/engine.hpp"

int main() {
  ocl::Engine engine("print_info", {1024});
  engine.setLocalWorkSizes({64});
  engine.run();
}