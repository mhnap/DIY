#ifndef SRC_PRACTICES_MATRIX_TRANSFORM_ROTATE_HPP
#define SRC_PRACTICES_MATRIX_TRANSFORM_ROTATE_HPP

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

} // namespace matrix::transform

#endif // SRC_PRACTICES_MATRIX_TRANSFORM_ROTATE_HPP