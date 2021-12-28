#include "common/utils.hpp"
#include <utility>
#include <vector>

int main() {
  std::vector<std::vector<int>> matrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
      {16, 17, 18, 19, 20},
      {21, 22, 23, 24, 25},
  }};

  int start = 0;
  int end = matrix.size() - 1;

  for (; start < end; ++start, --end) {
    for (int index = 0; start + index < end; ++index) {
      int value = matrix[start][start + index];
      std::swap(matrix[start + index][end], value);
      std::swap(matrix[end][end - index], value);
      std::swap(matrix[end - index][start], value);
      std::swap(matrix[start][start + index], value);
    }
  }

  common::printMatrix(matrix);

  return 0;
}