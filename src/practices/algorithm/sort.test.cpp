#include "sort.hpp"
#include "common/utils.hpp"
#include <gtest/gtest.h>

namespace {

template <typename F>
struct Test : testing::Test {
  F func;
};

template <typename F>
struct TestPair : testing::Test {
  F func;
};

using DataType = float;
using AlgorithmTypes = testing::Types<
    common::Functor<algorithm::sort::v1<DataType>>, common::Functor<algorithm::sort::v2<DataType>>,
    common::Functor<algorithm::sort::v3<DataType>>, common::Functor<algorithm::sort::v4<DataType>>,
    common::Functor<algorithm::sort::v5<DataType>>, common::Functor<algorithm::sort::v6<DataType>>>;
using AlgorithmTypesPair =
    testing::Types<common::Functor<algorithm::sort::v1<std::pair<size_t, DataType>, DataType>>,
                   common::Functor<algorithm::sort::v2<std::pair<size_t, DataType>, DataType>>,
                   common::Functor<algorithm::sort::v3<std::pair<size_t, DataType>, DataType>>,
                   common::Functor<algorithm::sort::v4<std::pair<size_t, DataType>, DataType>>,
                   common::Functor<algorithm::sort::v5<std::pair<size_t, DataType>, DataType>>,
                   common::Functor<algorithm::sort::v6<std::pair<size_t, DataType>, DataType>>>;
TYPED_TEST_SUITE(Test, AlgorithmTypes);
TYPED_TEST_SUITE(TestPair, AlgorithmTypesPair);

TYPED_TEST(Test, 1) {
  const std::vector<DataType> inVec = {1};
  const std::vector<DataType> outVec = {1};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 2) {
  const std::vector<DataType> inVec;
  const std::vector<DataType> outVec;
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 3) {
  const std::vector<DataType> inVec = {1, 2, 3, 4, 5};
  const std::vector<DataType> outVec = {1, 2, 3, 4, 5};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 4) {
  const std::vector<DataType> inVec = {5, 4, 3, 2, 1};
  const std::vector<DataType> outVec = {1, 2, 3, 4, 5};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 5) {
  const std::vector<DataType> inVec = {1.1, 2.3, 1.2, 1.1, 3.2, 1.3, 5.0};
  const std::vector<DataType> outVec = {1.1, 1.1, 1.2, 1.3, 2.3, 3.2, 5.0};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(Test, 6) {
  const std::vector<DataType> inVec = {5.0, 1.3, 3.2, 1.1, 1.2, 2.3, 1.1};
  const std::vector<DataType> outVec = {1.1, 1.1, 1.2, 1.3, 2.3, 3.2, 5.0};
  EXPECT_EQ(this->func(inVec), outVec);
}

TYPED_TEST(TestPair, 1) {
  const std::vector<std::pair<size_t, DataType>> inVec = {{1, 2.3}, {2, 1.1}, {3, 1.2}, {4, 3.2},
                                                          {5, 1.1}, {6, 1.3}, {7, 5.0}, {8, 1.1}};
  const std::vector<std::pair<size_t, DataType>> outVec = {{2, 1.1}, {5, 1.1}, {8, 1.1}, {3, 1.2},
                                                           {6, 1.3}, {1, 2.3}, {4, 3.2}, {7, 5.0}};
  EXPECT_EQ(this->func(inVec), outVec);
}

} // namespace
