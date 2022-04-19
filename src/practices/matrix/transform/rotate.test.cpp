#include "rotate.hpp"
#include <gtest/gtest.h>

namespace {

template <typename Func>
class RotateTest : public testing::Test {
protected:
  Func rotate;
};

class RotateFunctor {
public:
  template <typename T>
  void operator()(T& matrix) const {
    matrix::transform::rotate(matrix);
  }
};

class RotateParallelFunctor {
public:
  template <typename T>
  void operator()(T& matrix) const {
    matrix::transform::rotateParallel(matrix);
  }
};

using DataTypes = testing::Types<RotateFunctor, RotateParallelFunctor>;
TYPED_TEST_SUITE(RotateTest, DataTypes);

TYPED_TEST(RotateTest, four_times) {
  std::vector<std::vector<float>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
      {16, 17, 18, 19, 20},
      {21, 22, 23, 24, 25},
  }};
  auto outMatrix = inMatrix;
  for (int i = 0; i < 4; ++i) {
    this->rotate(outMatrix);
  }
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, five_to_five) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
      {16, 17, 18, 19, 20},
      {21, 22, 23, 24, 25},
  }};
  this->rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {21, 16, 11, 6, 1},
      {22, 17, 12, 7, 2},
      {23, 18, 13, 8, 3},
      {24, 19, 14, 9, 4},
      {25, 20, 15, 10, 5},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, four_to_four) {
  std::vector<std::vector<long>> inMatrix = {{
      {1, 2, 3, 4},
      {5, 6, 7, 8},
      {9, 10, 11, 12},
      {13, 14, 15, 16},
  }};
  this->rotate(inMatrix);
  std::vector<std::vector<long>> outMatrix = {{
      {13, 9, 5, 1},
      {14, 10, 6, 2},
      {15, 11, 7, 3},
      {16, 12, 8, 4},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, three_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3},
      {4, 5, 6},
      {7, 8, 9},
  }};
  this->rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix = {{
      {7, 4, 1},
      {8, 5, 2},
      {9, 6, 3},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, two_to_two) {
  std::vector<std::vector<short>> inMatrix = {{
      {1, 2},
      {3, 4},
  }};
  this->rotate(inMatrix);
  std::vector<std::vector<short>> outMatrix = {{
      {3, 1},
      {4, 2},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, one_to_one) {
  std::vector<std::vector<char>> inMatrix = {{
      {{1}},
  }};
  this->rotate(inMatrix);
  std::vector<std::vector<char>> outMatrix = {{
      {{1}},
  }};
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(RotateTest, zero) {
  std::vector<std::vector<int>> inMatrix;
  this->rotate(inMatrix);
  std::vector<std::vector<int>> outMatrix;
  EXPECT_EQ(inMatrix, outMatrix);
}

} // namespace