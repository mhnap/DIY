#include "transpose.hpp"
#include <gtest/gtest.h>

namespace {

template <typename Func>
class TransposeTest : public testing::Test {
protected:
  Func transpose;
};

class TransposeFunctor {
public:
  template <typename T>
  [[nodiscard]] auto operator()(const T& matrix) const {
    return matrix::transform::transpose(matrix);
  }
};

class TransposeParallelFunctor {
public:
  template <typename T>
  [[nodiscard]] auto operator()(const T& matrix) const {
    return matrix::transform::transposeParallel(matrix);
  }
};

using DataTypes = testing::Types<TransposeFunctor, TransposeParallelFunctor>;
TYPED_TEST_SUITE(TransposeTest, DataTypes);

TYPED_TEST(TransposeTest, four_times) {
  const std::vector<std::vector<char>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
  }};
  auto outMatrix = inMatrix;
  for (int i = 0; i < 4; ++i) {
    outMatrix = this->transpose(outMatrix);
  }
  EXPECT_EQ(inMatrix, outMatrix);
}

TYPED_TEST(TransposeTest, four_to_four) {
  const std::vector<std::vector<short>> inMatrix = {{
      {1, 2, 3, 4},
      {5, 6, 7, 8},
      {9, 10, 11, 12},
      {13, 14, 15, 16},
  }};
  const std::vector<std::vector<short>> outMatrix = {{
      {1, 5, 9, 13},
      {2, 6, 10, 14},
      {3, 7, 11, 15},
      {4, 8, 12, 16},
  }};
  EXPECT_EQ(this->transpose(inMatrix), outMatrix);
}

TYPED_TEST(TransposeTest, two_to_four) {
  const std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4},
      {5, 6, 7, 8},
  }};
  const std::vector<std::vector<int>> outMatrix = {{
      {1, 5},
      {2, 6},
      {3, 7},
      {4, 8},
  }};
  EXPECT_EQ(this->transpose(inMatrix), outMatrix);
}

TYPED_TEST(TransposeTest, four_to_two) {
  const std::vector<std::vector<long>> inMatrix = {{
      {1, 5},
      {2, 6},
      {3, 7},
      {4, 8},
  }};
  const std::vector<std::vector<long>> outMatrix = {{
      {1, 2, 3, 4},
      {5, 6, 7, 8},
  }};
  EXPECT_EQ(this->transpose(inMatrix), outMatrix);
}

TYPED_TEST(TransposeTest, three_to_five) {
  const std::vector<std::vector<float>> inMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
  }};
  const std::vector<std::vector<float>> outMatrix = {{
      {1, 6, 11},
      {2, 7, 12},
      {3, 8, 13},
      {4, 9, 14},
      {5, 10, 15},
  }};
  EXPECT_EQ(this->transpose(inMatrix), outMatrix);
}

TYPED_TEST(TransposeTest, five_to_three) {
  const std::vector<std::vector<char>> inMatrix = {{
      {1, 6, 11},
      {2, 7, 12},
      {3, 8, 13},
      {4, 9, 14},
      {5, 10, 15},
  }};
  const std::vector<std::vector<char>> outMatrix = {{
      {1, 2, 3, 4, 5},
      {6, 7, 8, 9, 10},
      {11, 12, 13, 14, 15},
  }};
  EXPECT_EQ(this->transpose(inMatrix), outMatrix);
}

TYPED_TEST(TransposeTest, one_to_one) {
  const std::vector<std::vector<short>> inMatrix = {{
      {{1}},
  }};
  EXPECT_EQ(this->transpose(inMatrix), inMatrix);
}

TYPED_TEST(TransposeTest, zero) {
  const std::vector<std::vector<int>> inMatrix;
  EXPECT_EQ(this->transpose(inMatrix), inMatrix);
}

} // namespace
