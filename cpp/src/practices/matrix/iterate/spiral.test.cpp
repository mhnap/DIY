#include "spiral.hpp"
#include <gtest/gtest.h>

namespace {

TEST(spiral, five_to_four) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4},
      {14, 15, 16, 5},
      {13, 20, 17, 6},
      {12, 19, 18, 7},
      {11, 10, 9, 8},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, four_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3},
      {10, 11, 4},
      {9, 12, 5},
      {8, 7, 6},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, three_to_four) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3, 4},
      {10, 11, 12, 5},
      {9, 8, 7, 6},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, three_to_one) {
  std::vector<std::vector<int>> inMatrix = {{
      {{1}},
      {{2}},
      {{3}},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1, 2, 3};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, one_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1, 2, 3};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, one_to_one) {
  std::vector<std::vector<int>> inMatrix = {{
      {{1}},
  }};
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList = {1};
  EXPECT_EQ(inList, outList);
}

TEST(spiral, zero_rows) {
  std::vector<std::vector<int>> inMatrix;
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList;
  EXPECT_EQ(inList, outList);
}

TEST(spiral, zero_columns) {
  std::vector<std::vector<int>> inMatrix;
  inMatrix.emplace_back();
  auto inList = matrix::iterate::spiral(inMatrix);
  std::vector<int> outList;
  EXPECT_EQ(inList, outList);
}

} // namespace