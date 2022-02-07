#include "common/object.hpp"
#include "common/utils.hpp"
#include <unordered_map>
#include <utility>

int main() {
  std::string withoutAutoStatistic;
  std::string withAutoStatistic;
  common::Object::disableLogs();

  // Fill map with values
  std::unordered_map<common::Object, int, common::Object::Hash> map;
  for (int i = 0; i < 10; ++i) {
    map.emplace(std::to_string(i), i);
  }
  common::Object::clearCounts();

  // Iterate without auto
  for (const std::pair<common::Object, int>& value : map) {
  }
  withoutAutoStatistic = common::Object::flushStatistic();

  // Iterate with auto
  for (const auto& value : map) {
  }
  withAutoStatistic = common::Object::flushStatistic();

  common::print("Without auto statistic : ", withoutAutoStatistic);
  common::print("With auto statistic    : ", withAutoStatistic);
}