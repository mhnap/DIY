#include "object.hpp"
#include "utils.hpp"

namespace common {

Object::Object() {
  ++defaultConstructCount;
  printIfEnabledLogs("default ctor");
}

Object::Object(std::string str) : m_str(std::move(str)) {
  ++userConstructCount;
  printIfEnabledLogs("user ctor");
}

Object::Object(const Object& o) : m_str(o.m_str) {
  ++copyConstructCount;
  printIfEnabledLogs("copy ctor");
}

Object::Object(Object&& o) noexcept : m_str(std::move(o.m_str)) {
  ++moveConstructCount;
  printIfEnabledLogs("move ctor");
  o.m_str = "moved";
}

Object& Object::operator=(const Object& o) {
  ++copyAssignmentCount;
  printIfEnabledLogs("copy assignment");
  m_str = o.m_str;
  return *this;
}

Object& Object::operator=(Object&& o) noexcept {
  ++moveAssignmentCount;
  printIfEnabledLogs("move assignment");
  m_str = std::move(o.m_str);
  o.m_str = "moved";
  return *this;
}

Object::~Object() {
  ++destructCount;
  printIfEnabledLogs("dtor");
}

void Object::enableLogs() { enabledLogs = true; }

void Object::disableLogs() { enabledLogs = false; }

void Object::clearCounts() {
  defaultConstructCount = 0;
  userConstructCount = 0;
  copyConstructCount = 0;
  moveConstructCount = 0;
  copyAssignmentCount = 0;
  moveAssignmentCount = 0;
  destructCount = 0;
}

std::string Object::flushStatistic() {
  auto statistic = concatToString("def_ctors[", defaultConstructCount, "] user_ctors[", userConstructCount,
                                  "] copy_ctors[", copyConstructCount, "] move_ctors[", moveConstructCount,
                                  "] copy_assigns[", copyAssignmentCount, "] move_assigns[",
                                  moveAssignmentCount, "] dtors[", destructCount, "]");
  clearCounts();
  return statistic;
}

std::size_t Object::Hash::operator()(const common::Object& o) const {
  return std::hash<std::string>()(o.m_str);
}

bool Object::operator==(const Object& o) const { return m_str == o.m_str; }

std::ostream& operator<<(std::ostream& os, const Object& o) { return os << o.m_str; }

void Object::printIfEnabledLogs(std::string_view log) {
  if (enabledLogs) {
    print(log);
  }
}

bool Object::enabledLogs = true;

std::size_t Object::defaultConstructCount = 0;
std::size_t Object::userConstructCount = 0;
std::size_t Object::copyConstructCount = 0;
std::size_t Object::moveConstructCount = 0;
std::size_t Object::copyAssignmentCount = 0;
std::size_t Object::moveAssignmentCount = 0;
std::size_t Object::destructCount = 0;

} // namespace common