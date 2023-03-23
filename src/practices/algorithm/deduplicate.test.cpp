#include "deduplicate.hpp"
#include "common/utils.hpp"
#include <gtest/gtest.h>

namespace {

template <typename F>
struct Test : testing::Test {
  F func;
};

using DataType = float;
using AlgorithmTypes = testing::Types<common::Functor<algorithm::deduplicate::v1<DataType>>,
                                      common::Functor<algorithm::deduplicate::v2<DataType>>,
                                      common::Functor<algorithm::deduplicate::v3<DataType>>,
                                      common::Functor<algorithm::deduplicate::v4<DataType>>>;
TYPED_TEST_SUITE(Test, AlgorithmTypes);

TYPED_TEST(Test, 1) {
  const std::vector<DataType> inVec = {1, 2, 3, 4, 5};
  const std::vector<DataType> outVec = {1, 2, 3, 4, 5};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 2) {
  const std::vector<DataType> inVec = {5, 4, 3, 2, 1};
  const std::vector<DataType> outVec = {5, 4, 3, 2, 1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 3) {
  const std::vector<DataType> inVec = {1, 1, 2, 3, 3};
  const std::vector<DataType> outVec = {1, 2, 3};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 4) {
  const std::vector<DataType> inVec = {1, 2, 2, 3, 4};
  const std::vector<DataType> outVec = {1, 2, 3, 4};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 5) {
  const std::vector<DataType> inVec = {3, 3, 3, 2, 1, 1, 2, 2, 3, 3};
  const std::vector<DataType> outVec = {3, 2, 1, 2, 3};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 6) {
  const std::vector<DataType> inVec = {3.1, 3.2, 3.2, 3.1, 1.5, 1.6, 1.6, 2.0, 2.0, 3.1};
  const std::vector<DataType> outVec = {3.1, 3.2, 3.1, 1.5, 1.6, 2.0, 3.1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 7) {
  const std::vector<DataType> inVec = {3.1, -3.2, 3.2, -3.1, 1.5, 1.6, 1.6, -2.0, -2.0, 3.1};
  const std::vector<DataType> outVec = {3.1, -3.2, 3.2, -3.1, 1.5, 1.6, -2.0, 3.1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 8) {
  const std::vector<DataType> inVec = {1, 1};
  const std::vector<DataType> outVec = {1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 9) {
  const std::vector<DataType> inVec = {1};
  const std::vector<DataType> outVec = {1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 10) {
  const std::vector<DataType> inVec;
  const std::vector<DataType> outVec;
  EXPECT_EQ(this->func(inVec), outVec);
}

TEST(MetadataTest, 1) {
  const std::vector<DataType> inVec = {1, 3, 3, 5, 2, 2, 4};
  const std::vector<DataType> refResult = {1, 3, 5, 2, 4};
  const std::vector<size_t> refIndices = {0, 1, 3, 4, 6};
  const std::vector<size_t> refRevIndices = {0, 1, 1, 2, 3, 3, 4};
  const std::vector<size_t> refOccurrences = {1, 2, 1, 2, 1};
  auto [result, indices, revIndices, occurrences] = algorithm::deduplicate::v5(inVec);
  EXPECT_EQ(result, refResult);
  EXPECT_EQ(indices, refIndices);
  EXPECT_EQ(revIndices, refRevIndices);
  EXPECT_EQ(occurrences, refOccurrences);
}

} // namespace
