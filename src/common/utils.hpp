#ifndef SRC_COMMON_UTILS_HPP
#define SRC_COMMON_UTILS_HPP

#include <iostream>
#include <sstream>
#include <string>

namespace common {

template <typename... Args>
void print(Args&&... args) {
  (std::cout << ... << std::forward<Args>(args));
  std::cout << std::endl;
}

template <typename... Args>
std::string concatToString(Args&&... args) {
  std::ostringstream ss;
  (ss << ... << std::forward<Args>(args));
  return ss.str();
}

} // namespace common

#endif // SRC_COMMON_UTILS_HPP