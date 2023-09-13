#include "common/object.hpp"
#include "common/utils.hpp"
#include <vector>

int main() {
  std::string pushBackObjectStatistic;
  std::string emplaceBackObjectStatistic;

  common::Object::disableLogs();
  common::Object object;
  common::Object::clearCounts();

  // Using push_back
  {
    std::vector<common::Object> vec;
    // Reserve some memory to not broke statistic during vector reallocation
    vec.reserve(10);
    vec.push_back(common::Object());
    vec.push_back(object);
    vec.push_back(std::move(object));
  }
  pushBackObjectStatistic = common::Object::flushStatistic();

  // Using emplace_back
  {
    std::vector<common::Object> vec;
    // Reserve some memory to not broke statistic during vector reallocation
    vec.reserve(10);
    vec.emplace_back(common::Object());
    vec.emplace_back(object);
    vec.emplace_back(std::move(object));
  }
  emplaceBackObjectStatistic = common::Object::flushStatistic();

  // No difference for already constructed objects
  common::println("push_back for constructed Object statistic    : ", pushBackObjectStatistic);
  common::println("emplace_back for constructed Object statistic : ", emplaceBackObjectStatistic);

  // Using push_back
  {
    std::vector<common::Object> vec;
    // Reserve some memory to not broke statistic during vector reallocation
    vec.reserve(10);
    // vec.push_back("object");
    vec.push_back({"object"});
  }
  pushBackObjectStatistic = common::Object::flushStatistic();

  // Using emplace_back
  {
    std::vector<common::Object> vec;
    // Reserve some memory to not broke statistic during vector reallocation
    vec.reserve(10);
    vec.emplace_back("object");
  }
  emplaceBackObjectStatistic = common::Object::flushStatistic();

  // Object is already constructed for emplace_back case
  common::println("push_back for no Object statistic    : ", pushBackObjectStatistic);
  common::println("emplace_back for no Object statistic : ", emplaceBackObjectStatistic);
}