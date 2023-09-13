#include "shortest_matching_substring.hpp"
#include "common/utils.hpp"
#include <gtest/gtest.h>

namespace {

template <typename F>
struct Test : testing::Test {
  F func;
};

using AlgorithmTypes = testing::Types<common::Functor<algorithm::shortest_matching_substring::v1>,
                                      common::Functor<algorithm::shortest_matching_substring::v2>>;
TYPED_TEST_SUITE(Test, AlgorithmTypes);

TYPED_TEST(Test, 1) {
  const auto string = "abcde";
  const auto substring = "cc";
  EXPECT_EQ(this->func(string, substring), "");
}

TYPED_TEST(Test, 2) {
  const auto string = "abcde";
  const auto substring = "ce";
  EXPECT_EQ(this->func(string, substring), "cde");
}

TYPED_TEST(Test, 3) {
  const auto string = "abacedaectycbayedqddaectycbayacedaectdaecbaycedaectcedaec";
  const auto substring = "cbayd";
  EXPECT_EQ(this->func(string, substring), "cbayed");
}

TYPED_TEST(Test, 4) {
  const auto string = "abacedacbayedqd";
  const auto substring = "ecc";
  EXPECT_EQ(this->func(string, substring), "cedac");
}

TYPED_TEST(Test, 5) {
  const auto string = "aaa";
  const auto substring = "a";
  EXPECT_EQ(this->func(string, substring), "a");
}

TYPED_TEST(Test, 6) {
  const auto string = "a";
  const auto substring = "a";
  EXPECT_EQ(this->func(string, substring), "a");
}

TYPED_TEST(Test, 7) {
  const auto string = "";
  const auto substring = "a";
  EXPECT_EQ(this->func(string, substring), "");
}

TYPED_TEST(Test, 8) {
  const auto string = "a";
  const auto substring = "";
  EXPECT_EQ(this->func(string, substring), "");
}

TYPED_TEST(Test, 9) {
  const auto string = "";
  const auto substring = "";
  EXPECT_EQ(this->func(string, substring), "");
}

TYPED_TEST(Test, 10) {
  const auto string = "aa";
  const auto substring = "aa";
  EXPECT_EQ(this->func(string, substring), "aa");
}

TYPED_TEST(Test, 11) {
  const auto string = "ab";
  const auto substring = "b";
  EXPECT_EQ(this->func(string, substring), "b");
}

} // namespace
