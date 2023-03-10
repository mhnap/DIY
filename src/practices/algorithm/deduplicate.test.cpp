#include "deduplicate.hpp"
#include <gtest/gtest.h>

namespace {

TEST(deduplicate, 1) {
  const std::vector<int> inVec = {1, 2, 3, 4, 5};
  const std::vector<int> outVec = {1, 2, 3, 4, 5};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 2) {
  const std::vector<int> inVec = {5, 4, 3, 2, 1};
  const std::vector<int> outVec = {5, 4, 3, 2, 1};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 3) {
  const std::vector<int> inVec = {1, 1, 2, 3, 3};
  const std::vector<int> outVec = {1, 2, 3};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 4) {
  const std::vector<int> inVec = {1, 2, 2, 3, 4};
  const std::vector<int> outVec = {1, 2, 3, 4};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 5) {
  const std::vector<int> inVec = {3, 3, 3, 2, 1, 1, 2, 2, 3, 3};
  const std::vector<int> outVec = {3, 2, 1, 2, 3};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 6) {
  const std::vector<float> inVec = {3.1, 3.2, 3.2, 3.1, 1.5, 1.6, 1.6, 2.0, 2.0, 3.1};
  const std::vector<float> outVec = {3.1, 3.2, 3.1, 1.5, 1.6, 2.0, 3.1};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

TEST(deduplicate, 7) {
  const std::vector<float> inVec = {3.1, -3.2, 3.2, -3.1, 1.5, 1.6, 1.6, -2.0, -2.0, 3.1};
  const std::vector<float> outVec = {3.1, -3.2, 3.2, -3.1, 1.5, 1.6, -2.0, 3.1};
  const auto resultStd = algorithm::deduplicateStd(inVec);
  const auto result = algorithm::deduplicate(inVec);
  EXPECT_EQ(resultStd, outVec);
  EXPECT_EQ(result, outVec);
}

} // namespace
