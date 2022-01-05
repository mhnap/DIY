#include "trade_matching_engine.hpp"
#include <gtest/gtest.h>

namespace {

TEST(assignment, example) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 5 30\n"
        << "T2 S 5 70\n"
        << "T3 B 1 40\n"
        << "T4 S 2 60\n"
        << "T5 S 3 70\n"
        << "T6 S 20 80\n"
        << "T7 S 1 50\n"
        << "T2 S 5 70\n"
        << "T1 B 1 50\n"
        << "T1 B 3 60\n"
        << "T7 S 2 50\n"
        << "T8 B 10 90\n";
  engine.run(input);
  auto actual = engine.getSerializedTrades();
  std::string expected = "T1+1@50 T7-1@50\n"
                         "T1+2@60 T4-2@60\n"
                         "T1+1@60 T7-1@60\n"
                         "T2-6@70 T5-3@70 T7-1@50 T8+1@50 T8+9@70\n";
  EXPECT_EQ(actual, expected);
}

TEST(equal_price, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 50\n"
        << "T2 S 1 50\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1+1@50 T2-1@50\n";
  EXPECT_EQ(actual, expected);
}

TEST(equal_price, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 50\n"
        << "T2 B 1 50\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1-1@50 T2+1@50\n";
  EXPECT_EQ(actual, expected);
}

TEST(better_price, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 50\n"
        << "T2 S 1 10\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1+1@50 T2-1@50\n";
  EXPECT_EQ(actual, expected);
}

TEST(better_price, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 10\n"
        << "T2 B 1 50\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1-1@10 T2+1@10\n";
  EXPECT_EQ(actual, expected);
}

TEST(worse_price, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 10\n"
        << "T2 S 1 50\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected;
  EXPECT_EQ(actual, expected);
}

TEST(worse_price, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 50\n"
        << "T2 B 1 10\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected;
  EXPECT_EQ(actual, expected);
}

TEST(best_price_first, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 20\n"
        << "T2 B 1 30\n"
        << "T3 S 1 10\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T2+1@30 T3-1@30\n";
  EXPECT_EQ(actual, expected);
}

TEST(best_price_first, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 20\n"
        << "T2 S 1 10\n"
        << "T3 B 1 30\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T2-1@10 T3+1@10\n";
  EXPECT_EQ(actual, expected);
}

TEST(oldest_first, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 30\n"
        << "T2 B 1 30\n"
        << "T3 S 1 10\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1+1@30 T3-1@30\n";
  EXPECT_EQ(actual, expected);
}

TEST(oldest_first, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 10\n"
        << "T2 S 1 10\n"
        << "T3 B 1 30\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1-1@10 T3+1@10\n";
  EXPECT_EQ(actual, expected);
}

TEST(multiple_trades, sell) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 B 1 30\n"
        << "T1 B 1 30\n"
        << "T2 B 1 30\n"
        << "T3 S 3 10\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1+2@30 T2+1@30 T3-3@30\n";
  EXPECT_EQ(actual, expected);
}

TEST(multiple_trades, buy) {
  trade_matching::Engine engine;
  std::stringstream input;
  input << "T1 S 1 10\n"
        << "T1 S 1 10\n"
        << "T2 S 1 10\n"
        << "T3 B 3 30\n";
  engine.run(input);
  std::string actual = engine.getSerializedTrades();
  std::string expected = "T1-2@10 T2-1@10 T3+3@10\n";
  EXPECT_EQ(actual, expected);
}

} // namespace