#ifndef SRC_COMMON_OBJECT_HPP
#define SRC_COMMON_OBJECT_HPP

#include <string>
#include <string_view>

namespace common {

class Object {
public:
  Object();
  Object(std::string str);
  Object(const Object& o);
  Object(Object&& o) noexcept;
  Object& operator=(const Object& o);
  Object& operator=(Object&& o) noexcept;
  ~Object();

  static void enableLogs();
  static void disableLogs();

  static void clearCounts();
  static std::string flushStatistic();

  friend std::ostream& operator<<(std::ostream& os, const Object& o);

private:
  void printIfEnabledLogs(std::string_view log);

private:
  std::string m_str = "empty";

  static bool enabledLogs;

  static std::size_t defaultConstructCount;
  static std::size_t userConstructCount;
  static std::size_t copyConstructCount;
  static std::size_t moveConstructCount;
  static std::size_t copyAssignmentCount;
  static std::size_t moveAssignmentCount;
  static std::size_t destructCount;
};

} // namespace common

#endif // SRC_COMMON_OBJECT_HPP