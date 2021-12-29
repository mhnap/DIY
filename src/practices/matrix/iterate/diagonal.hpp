#ifndef SRC_PRACTICES_MATRIX_ITERATE_DIAGONAL_HPP
#define SRC_PRACTICES_MATRIX_ITERATE_DIAGONAL_HPP

#include <vector>

namespace matrix::iterate {

template <typename T>
auto diagonal(const std::vector<std::vector<T>>& matrix) {
  std::vector<T> values;

  if (matrix.empty()) {
    return values;
  }

  int rowCount = matrix.size();
  int columnCount = matrix[0].size();
  for (int I = rowCount - 1, J = 0; I != 0 || J != columnCount; I == 0 ? ++J : --I) {
    for (int i = I, j = J; i < rowCount && j < columnCount; ++i, ++j) {
      values.push_back(matrix[i][j]);
    }
  }

  return values;
}

} // namespace matrix::iterate

#endif // SRC_PRACTICES_MATRIX_ITERATE_DIAGONAL_HPP