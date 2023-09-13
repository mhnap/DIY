#include "ocl/core/engine.hpp"
#include <array>
#include <iostream>

int main() {
  constexpr size_t SIZE_ROW = 16;
  constexpr size_t SIZE_COL = 64;
  constexpr size_t TOTAL_SIZE = SIZE_ROW * SIZE_COL;
  std::array<std::array<int, SIZE_COL>, SIZE_ROW> data;
  decltype(data) results;
  size_t correct = 0;

  // Fill our data set with random float values
  for (size_t i = 0; i < SIZE_ROW; ++i) {
    for (size_t j = 0; j < SIZE_COL; ++j) {
      data[i][j] = rand() % 1024;
    }
  }

  // Calculate math square by running OpenCL kernel
  ocl::Engine engine("math_square", {TOTAL_SIZE});
  engine.setData(&data, &results, TOTAL_SIZE, ocl::DataType::Int);
  engine.run();

  // Validate our results
  for (size_t i = 0; i < SIZE_ROW; ++i) {
    for (size_t j = 0; j < SIZE_COL; ++j) {
      if (results[i][j] == data[i][j] * data[i][j]) {
        ++correct;
      }
    }
  }

  // Print a brief summary detailing the results
  std::cout << "Computed " << correct << '/' << TOTAL_SIZE << " correct values!" << std::endl;
}