#pragma once

#include <functional>
#include <iostream>
#include <sstream>
#include <string>

namespace common {

template <typename... Args>
void print(Args&&... args) {
  (std::cout << ... << std::forward<Args>(args));
}

template <typename... Args>
void println(Args&&... args) {
  print(args...);
  std::cout << std::endl;
}

template <typename... Args>
std::string concatToString(Args&&... args) {
  std::ostringstream ss;
  (ss << ... << std::forward<Args>(args));
  return ss.str();
}

template <auto F>
struct Functor {
  template <typename... Args>
  auto operator()(Args&&... args) const {
    return std::invoke(F, std::forward<Args>(args)...);
  }
};

} // namespace common