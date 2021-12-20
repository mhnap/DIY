#include <array>
#include <iostream>

int main() {
  constexpr int rowCount = 4;
  constexpr int columnCount = 3;
  std::array<std::array<int, columnCount>, rowCount> matrix{{
      {6, 9, 11},
      {3, 7, 10},
      {1, 4, 8},
      {0, 2, 5},
  }};

  for (auto I = rowCount - 1, J = 0; I != 0 || J != columnCount; I == 0 ? ++J : --I) {
    for (auto i = I, j = J; i < rowCount && j < columnCount; ++i, ++j) {
      std::cout << matrix[i][j] << std::endl;
    }
  }
}