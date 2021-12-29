#ifndef SRC_PRACTICES_MATRIX_ITERATE_SPIRAL_HPP
#define SRC_PRACTICES_MATRIX_ITERATE_SPIRAL_HPP

#include <vector>

namespace matrix::iterate {

template <typename T>
auto spiral(const std::vector<std::vector<T>>& matrix) {
  std::vector<T> values;

  if (matrix.empty()) {
    return values;
  }

  int rowStart = 0;
  int columnStart = 0;
  int rowEnd = matrix.size() - 1;
  int columnEnd = matrix[0].size() - 1;

  enum class Direction { Right, Down, Left, Up };
  std::vector<Direction> directions = {Direction::Right, Direction::Down, Direction::Left, Direction::Up};
  int directionEnd = directions.size() - 1;

  for (int direction = 0; rowStart <= rowEnd && columnStart <= columnEnd;
       direction == directionEnd ? direction = 0 : ++direction) {
    switch (directions[direction]) {
    case Direction::Right:
      for (int column = columnStart; column <= columnEnd; ++column) {
        values.push_back(matrix[rowStart][column]);
      }
      ++rowStart;
      break;
    case Direction::Down:
      for (int row = rowStart; row <= rowEnd; ++row) {
        values.push_back(matrix[row][columnEnd]);
      }
      --columnEnd;
      break;
    case Direction::Left:
      for (int column = columnEnd; column >= columnStart; --column) {
        values.push_back(matrix[rowEnd][column]);
      }
      --rowEnd;
      break;
    case Direction::Up:
      for (int row = rowEnd; row >= rowStart; --row) {
        values.push_back(matrix[row][columnStart]);
      }
      ++columnStart;
      break;
    }
  }

  return values;
}

} // namespace matrix::iterate

#endif // SRC_PRACTICES_MATRIX_ITERATE_SPIRAL_HPP