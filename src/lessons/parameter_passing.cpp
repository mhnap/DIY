#include "common/object.hpp"
#include "common/utils.hpp"
#include <array>
#include <vector>

template <typename T>
class WithRef {
public:
  explicit WithRef(const T& d) : data(d) {}

private:
  T data;
};

template <typename T>
class WithMove {
public:
  explicit WithMove(T d) : data(std::move(d)) {}

private:
  T data;
};

template <typename T>
void runTests(const T& d) {
  common::Object::clearCounts();
  // Using with ref
  {
    T data = d;
    WithRef withRef(data);
  }
  const auto withRefStatistic = common::Object::flushStatistic();

  // Using with ref and move
  {
    T data = d;
    WithRef withRef(std::move(data));
  }
  const auto withRefAndMoveStatistic = common::Object::flushStatistic();

  // Using with move
  {
    T data = d;
    WithMove withMove(data);
  }
  const auto withMoveStatistic = common::Object::flushStatistic();

  // Using with move and move
  {
    T data = d;
    WithMove withMove(std::move(data));
  }
  const auto withMoveAndMoveStatistic = common::Object::flushStatistic();

  // Print results
  common::print("withRefStatistic         : ", withRefStatistic);
  common::print("withRefAndMoveStatistic  : ", withRefAndMoveStatistic);
  common::print("withMoveStatistic        : ", withMoveStatistic);
  common::print("withMoveAndMoveStatistic : ", withMoveAndMoveStatistic);
}

int main() {
  common::Object::disableLogs();

  common::print("\n----- Tests results for object type -----");
  const common::Object obj;
  runTests(obj);

  common::print("\n----- Tests results for vector type -----");
  const std::vector<common::Object> vec(1);
  runTests(vec);

  common::print("\n----- Tests results for array type -----");
  const std::array<common::Object, 1> arr;
  runTests(arr);
}
