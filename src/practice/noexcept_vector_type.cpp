#include "common/except_object.hpp"
#include "common/object.hpp"
#include "common/utils.hpp"
#include <vector>

int main() {
  const int ObjectCount = 100;

  std::string withReserveObjectStatistic;
  std::string withoutReserveExceptObjectStatistic;
  std::string withoutReserveNoexceptObjectStatistic;

  // No need for logs in this practice
  common::Object::disableLogs();

  // Vector with reserve won't reallocate objects during growth
  {
    std::vector<common::Object> vector;
    vector.reserve(ObjectCount);
    for (int i = 0; i < ObjectCount; ++i) {
      vector.emplace_back("object");
    }
  }
  withReserveObjectStatistic = common::Object::flushStatistic();

  // Vector without reserve will reallocate objects during growth
  // And copy all objects if move ctor is not specified as noexcept
  {
    std::vector<common::ExceptObject> vector;
    // vector.reserve(ObjectCount);
    for (int i = 0; i < ObjectCount; ++i) {
      vector.emplace_back("object");
    }
  }
  withoutReserveExceptObjectStatistic = common::Object::flushStatistic();

  // Vector without reserve will reallocate objects during growth
  // And move all objects if move ctor is specified as noexcept
  {
    std::vector<common::Object> vector;
    // vector.reserve(ObjectCount);
    for (int i = 0; i < ObjectCount; ++i) {
      vector.emplace_back("object");
    }
  }
  withoutReserveNoexceptObjectStatistic = common::Object::flushStatistic();

  common::print("With reserve Object statistic             : ", withReserveObjectStatistic);
  common::print("Without reserve Except Object statistic   : ", withoutReserveExceptObjectStatistic);
  common::print("Without reserve Noexcept Object statistic : ", withoutReserveNoexceptObjectStatistic);
}