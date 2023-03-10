#include "deduplicate.hpp"
#include <gtest/gtest.h>

namespace {

TEST(deduplicate, 1) {
  const std::vector<int> inVec = {1, 2, 3, 4, 5};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

TEST(deduplicate, 2) {
  const std::vector<int> inVec = {5, 4, 3, 2, 1};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

TEST(deduplicate, 3) {
  const std::vector<int> inVec = {1, 1, 2, 3, 3};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

TEST(deduplicate, 4) {
  const std::vector<int> inVec = {1, 2, 2, 3, 4};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

TEST(deduplicate, 5) {
  const std::vector<int> inVec = {3, 3, 3, 2, 1, 1, 2, 2, 3, 3};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

TEST(deduplicate, 6) {
  const std::vector<float> inVec = {3.1, 3.2, 3.2, 3.1, 1.5, 1.6, 1.6, 2.0, 2.0, 3.1};
  EXPECT_EQ(algorithm::deduplicateStd(inVec), algorithm::deduplicate(inVec));
}

} // namespace
