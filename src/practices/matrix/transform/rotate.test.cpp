#include "rotate.hpp"
#include <gtest/gtest.h>

namespace {

TEST(rotate, four_times) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
      {16, 17, 18, 19, 20},
      {21, 22, 23, 24, 25},
  }};
  auto outMatrix = inMatrix;
  for (int i = 0; i < 4; ++i) {
    matrix::transform::rotate(outMatrix);
  }
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, five_to_five) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
      {16, 17, 18, 19, 20},
      {21, 22, 23, 24, 25},
  }};
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {21, 16, 11, 6, 1},
      {22, 17, 12, 7, 2},
      {23, 18, 13, 8, 3},
      {24, 19, 14, 9, 4},
      {25, 20, 15, 10, 5},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, four_to_four) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4},
      {5, 6, 7, 8},
      {9, 10, 11, 12},
      {13, 14, 15, 16},
  }};
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {13, 9, 5, 1},
      {14, 10, 6, 2},
      {15, 11, 7, 3},
      {16, 12, 8, 4},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, three_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3},
      {4, 5, 6},
      {7, 8, 9},
  }};
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {7, 4, 1},
      {8, 5, 2},
      {9, 6, 3},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, two_to_two) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2},
      {3, 4},
  }};
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {3, 1},
      {4, 2},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, one_to_one) {
  std::vector<std::vector<int>> inMatrix = {{
      {{1}},
  }};
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {{1}},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TEST(rotate, zero) {
  std::vector<std::vector<int>> inMatrix;
  matrix::transform::rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix;
  EXPECT_EQ(inMatrix, outMatrix);
}

} // namespace