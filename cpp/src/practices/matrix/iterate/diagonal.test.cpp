#include "diagonal.hpp"
#include <gtest/gtest.h>

namespace {

TEST(diagonal, four_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {7, 10, 12},
      {4, 8, 11},
      {2, 5, 9},
      {1, 3, 6},
  }};
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, three_to_four) {
  std::vector<std::vector<int>> inMatrix = {{
      {4, 7, 10, 12},
      {2, 5, 8, 11},
      {1, 3, 6, 9},
  }};
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, three_to_one) {
  std::vector<std::vector<int>> inMatrix = {{
      {{3}},
      {{2}},
      {{1}},
  }};
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList = {1, 2, 3};
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, one_to_three) {
  std::vector<std::vector<int>> inMatrix = {{
      {1, 2, 3},
  }};
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList = {1, 2, 3};
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, one_to_one) {
  std::vector<std::vector<int>> inMatrix = {{
      {{1}},
  }};
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList = {1};
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, zero_rows) {
  std::vector<std::vector<int>> inMatrix;
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList;
  EXPECT_EQ(inList, outList);
}

TEST(diagonal, zero_columns) {
  std::vector<std::vector<int>> inMatrix;
  inMatrix.emplace_back();
  auto inList = matrix::iterate::diagonal(inMatrix);
  std::vector<int> outList;
  EXPECT_EQ(inList, outList);
}

} // namespace