#pragma once

#include "ocl/core/engine.hpp"
#include "ocl/core/utils.hpp"
#include <utility>
#include <vector>

namespace matrix::transform {

template <typename T>
void rotate(std::vector<std::vector<T>>& matrix) {
  for (int start = 0, end = matrix.size() - 1; start < end; ++start, --end) {
    for (int index = 0; start + index < end; ++index) {
      auto value = matrix[start][start + index];
      std::swap(matrix[start + index][end], value);
      std::swap(matrix[end][end - index], value);
      std::swap(matrix[end - index][start], value);
      std::swap(matrix[start][start + index], value);
    }
  }
}

template <typename T>
void rotateParallel(std::vector<std::vector<T>>& matrix) {
  if (matrix.empty()) {
    return;
  }

  // Convert to flat data
  const auto SIZE = matrix.size();
  auto flatData = ocl::convert2dTo1d(matrix);
  decltype(flatData) flatResults(SIZE * SIZE);

  // Rotate matrix by running OpenCL kernel
  ocl::Engine engine("matrix_rotate", {SIZE, SIZE});
  engine.setData(flatData.data(), flatResults.data(), SIZE * SIZE, ocl::dataTypeFromType<T>());
  engine.addCompilerDefineOption("SIZE", std::to_string(SIZE));
  engine.run();
  ocl::convert1dTo2d(flatResults, matrix);
}

} // namespace matrix::transform