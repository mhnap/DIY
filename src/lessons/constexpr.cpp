#include "common/utils.hpp"
#include <array>
#include <random>

constexpr int myPow(int base, int exp) noexcept { return (exp == 0 ? 1 : base * myPow(base, exp - 1)); }

int getRandom(int biggest) {
  std::random_device dev;
  std::mt19937 rng(dev());
  std::uniform_int_distribution<int> dist(1, biggest);
  return dist(rng);
}

int main() {
  //  int size = 10;
  //  // Non-type template argument is not a constant expression
  //  std::array<int, size> arr;

  constexpr int base = 4;
  constexpr int exp = 4;

  // In this case myPow is computed at compile time
  std::array<int, myPow(base, exp)> arr{};
  common::println("Array size: ", arr.size());

  // In this case myPow is computed at runtime
  auto randomForBase = getRandom(base);
  auto randomForExp = getRandom(exp);
  auto powResult = myPow(randomForBase, randomForExp);
  common::println("Pow result for ", randomForBase, '^', randomForExp, ": ", powResult);
}