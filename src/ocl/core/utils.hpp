#pragma once

#include <vector>

namespace ocl {

template <typename T>
auto convert2dTo1d(const std::vector<std::vector<T>>& input) {
  std::vector<T> output;
  if (input.empty()) {
    return output;
  }

  const auto rowSize = input.size();
  const auto columnSize = input[0].size();
  output.resize(rowSize * columnSize);

  for (size_t i = 0; i < rowSize; ++i) {
    std::copy(input[i].begin(), input[i].end(), output.begin() + i * columnSize);
  }
  return output;
}

template <typename T>
auto convert1dTo2d(const std::vector<T>& input, size_t rowSize, size_t columnSize) {
  std::vector<std::vector<T>> output;
  if (input.empty()) {
    return output;
  }

  output.resize(rowSize);

  for (size_t i = 0; i < rowSize; ++i) {
    output[i].resize(columnSize);
    auto itPos = input.begin() + i * columnSize;
    std::copy(itPos, itPos + columnSize, output[i].begin());
  }
  return output;
}

template <typename T>
void convert1dTo2d(const std::vector<T>& input, std::vector<std::vector<T>>& output) {
  if (input.empty()) {
    return;
  }

  const auto rowSize = output.size();
  const auto columnSize = output[0].size();

  for (size_t i = 0; i < rowSize; ++i) {
    auto itPos = input.begin() + i * columnSize;
    std::copy(itPos, itPos + columnSize, output[i].begin());
  }
}

} // namespace ocl