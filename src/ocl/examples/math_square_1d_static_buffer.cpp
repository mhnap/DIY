#include "ocl/core/engine.hpp"
#include <array>
#include <iostream>

int main() {
  constexpr size_t SIZE = 1024;
  std::array<float, SIZE> data;
  decltype(data) results;
  size_t correct = 0;

  // Fill our data set with random float values
  for (auto& i : data) {
    i = rand() / (float)RAND_MAX;
  }

  // Calculate math square by running OpenCL kernel
  ocl::Engine engine("math_square", {SIZE});
  engine.setData(&data, &results, SIZE, ocl::DataType::Float);
  engine.run();

  // Validate our results
  for (auto& i : results) {
    if (results[i] == data[i] * data[i]) {
      ++correct;
    }
  }

  // Print a brief summary detailing the results
  std::cout << "Computed " << correct << '/' << SIZE << " correct values!" << std::endl;
}