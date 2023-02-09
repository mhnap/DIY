#pragma once

#include "ocl/core/engine.hpp"
#include "ocl/core/utils.hpp"
#include <utility>
#include <vector>

namespace matrix::transform {

template <typename T>
[[nodiscard]] auto transpose(const std::vector<std::vector<T>>& matrix) {
  if (matrix.empty()) {
    return matrix;
  }

  const auto rowCount = matrix.size();
  const auto columnCount = matrix[0].size();
  std::vector<std::vector<T>> transposed_matrix;
  transposed_matrix.resize(columnCount, std::vector<T>(rowCount));
  for (auto i = decltype(rowCount){}; i < rowCount; ++i) {
    for (auto j = decltype(columnCount){}; j < columnCount; ++j) {
      transposed_matrix[j][i] = matrix[i][j];
    }
  }

  return transposed_matrix;
}

template <typename T>
[[nodiscard]] auto transposeParallel(const std::vector<std::vector<T>>& matrix) {
  if (matrix.empty()) {
    return matrix;
  }

  // Convert to flat data
  const auto flatData = ocl::convert2dTo1d(matrix);
  const auto totalSize = flatData.size();
  auto flatResults = decltype(flatData)(totalSize);
  const auto rowSize = matrix.size();
  const auto columnSize = matrix[0].size();

  // Transpose matrix by running OpenCL kernel
  ocl::Engine engine("matrix_transpose", {rowSize, columnSize});
  engine.setData(flatData.data(), flatResults.data(), totalSize, ocl::dataTypeFromType<T>());
  engine.addCompilerOptionDefine("ROW_SIZE", std::to_string(rowSize));
  engine.addCompilerOptionDefine("COLUMN_SIZE", std::to_string(columnSize));
  engine.run();
  return ocl::convert1dTo2d(flatResults, columnSize, rowSize);
}

} // namespace matrix::transform
