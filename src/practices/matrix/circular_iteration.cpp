#include <iostream>
#include <vector>

int main() {
  std::vector<std::vector<int>> matrix = {{
      {1, 2, 3, 4},
      {14, 15, 16, 5},
      {13, 20, 17, 6},
      {12, 19, 18, 7},
      {11, 10, 9, 8},
  }};

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
        std::cout << matrix[rowStart][column] << std::endl;
      };
      ++rowStart;
      break;
    case Direction::Down:
      for (int row = rowStart; row <= rowEnd; ++row) {
        std::cout << matrix[row][columnEnd] << std::endl;
      };
      --columnEnd;
      break;
    case Direction::Left:
      for (int column = columnEnd; column >= columnStart; --column) {
        std::cout << matrix[rowEnd][column] << std::endl;
      };
      --rowEnd;
      break;
    case Direction::Up:
      for (int row = rowEnd; row >= rowStart; --row) {
        std::cout << matrix[row][columnStart] << std::endl;
      };
      ++columnStart;
      break;
    }
  }

  return 0;
}