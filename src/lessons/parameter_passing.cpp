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
class WithRvalueAndMove {
public:
  explicit WithRvalueAndMove(T&& d) : data(std::move(d)) {}

private:
  T data;
};

template <typename T>
void runTests(const T& d) {
  {
    T data = d;
    common::Object::clearCounts();
    WithRef withRef(data);
    common::println("withRefStatistic           : ", common::Object::flushStatistic());
  }

  {
    T data = d;
    common::Object::clearCounts();
    WithRef withRef(std::move(data));
    common::println("withRefAndMoveStatistic    : ", common::Object::flushStatistic());
  }

  {
    T data = d;
    common::Object::clearCounts();
    WithMove withMove(data);
    common::println("withMoveStatistic          : ", common::Object::flushStatistic());
  }

  {
    T data = d;
    common::Object::clearCounts();
    WithMove withMove(std::move(data));
    common::println("withMoveAndMoveStatistic   : ", common::Object::flushStatistic());
  }

  {
    T data = d;
    common::Object::clearCounts();
    WithRvalueAndMove withRvalueAndMove(std::move(data));
    common::println("withRvalueAndMoveStatistic : ", common::Object::flushStatistic());
  }
}

int main() {
  common::Object::disableLogs();

  common::println("\n----- Tests results for object type -----");
  const common::Object obj;
  runTests(obj);

  common::println("\n----- Tests results for vector type -----");
  const std::vector<common::Object> vec(1);
  runTests(vec);

  common::println("\n----- Tests results for array type -----");
  const std::array<common::Object, 1> arr;
  runTests(arr);
}
