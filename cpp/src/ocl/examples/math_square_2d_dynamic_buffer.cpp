#include "ocl/core/engine.hpp"
#include "ocl/core/utils.hpp"
#include <iostream>
#include <vector>

int main() {
  const size_t SIZE_ROW = 16;
  const size_t SIZE_COL = 64;
  const size_t TOTAL_SIZE = SIZE_ROW * SIZE_COL;
  std::vector<std::vector<long>> data(SIZE_ROW);
  size_t correct = 0;

  // Fill our data set with random float values
  for (size_t i = 0; i < SIZE_ROW; ++i) {
    data[i].resize(SIZE_COL);
    for (size_t j = 0; j < SIZE_COL; ++j) {
      data[i][j] = rand() % 1024;
    }
  }

  // Convert to flat data
  auto flatData = ocl::convert2dTo1d(data);
  decltype(flatData) flatResults(TOTAL_SIZE);

  // Calculate math square by running OpenCL kernel
  ocl::Engine engine("math_square", {TOTAL_SIZE});
  engine.setData(flatData.data(), flatResults.data(), TOTAL_SIZE, ocl::DataType::Long);
  engine.run();

  // Validate our results
  auto results = ocl::convert1dTo2d(flatResults, SIZE_ROW, SIZE_COL);
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