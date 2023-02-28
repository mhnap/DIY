#include "common/object.hpp"
#include "common/utils.hpp"
#include <vector>

class WithRef {
public:
  explicit WithRef(const std::vector<common::Object>& vec) : vec(vec) {}

private:
  std::vector<common::Object> vec;
};

class WithMove {
public:
  explicit WithMove(std::vector<common::Object> vec) : vec(std::move(vec)) {}

private:
  std::vector<common::Object> vec;
};

int main() {
  common::Object::disableLogs();

  // Using with ref
  {
    std::vector<common::Object> vec(10);
    WithRef withRefL(vec);
    WithRef withRefR(std::vector<common::Object>(10));
  }
  const auto withRefStatistic = common::Object::flushStatistic();

  // Using with ref and move
  {
    std::vector<common::Object> vec(10);
    WithRef withRefL(std::move(vec));
    WithRef withRefR(std::vector<common::Object>(10));
  }
  const auto withRefAndMoveStatistic = common::Object::flushStatistic();

  // Using with move
  {
    std::vector<common::Object> vec(10);
    WithMove withMoveL(vec);
    WithMove withMoveR(std::vector<common::Object>(10));
  }
  const auto withMoveStatistic = common::Object::flushStatistic();

  // Using with move and move
  {
    std::vector<common::Object> vec(10);
    WithMove withMoveL(std::move(vec));
    WithMove withMoveR(std::vector<common::Object>(10));
  }
  const auto withMoveAndMoveStatistic = common::Object::flushStatistic();

  // Print results
  common::print("withRefStatistic         : ", withRefStatistic);
  common::print("withRefAndMoveStatistic  : ", withRefAndMoveStatistic);
  common::print("withMoveStatistic        : ", withMoveStatistic);
  common::print("withMoveAndMoveStatistic : ", withMoveAndMoveStatistic);
}
