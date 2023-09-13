#pragma once

#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

namespace ocl {

class Error : public std::runtime_error {
public:
  Error(const std::initializer_list<std::string_view>& strs) : std::runtime_error(constructMsg(strs)) {}

  static std::string constructMsg(const std::vector<std::string_view>& strs) {
    std::string msg;
    if (strs.empty()) {
      return msg;
    }

    auto strIt = strs.begin();
    auto strEndIt = std::prev(strs.end());
    for (; strIt != strEndIt; ++strIt) {
      msg += *strIt;
      msg += delimiter;
    }
    msg += *strIt;
    msg += end;

    return msg;
  }

  static constexpr std::string_view delimiter = ": ";
  static constexpr std::string_view end = ".";
};

class OpenCLError : public Error {
public:
  explicit OpenCLError(std::string_view str, int error) : Error({getPrefix(), str, getErrorStr(error)}) {}
  static std::string getErrorStr(int error) { return "Error is " + std::to_string(error); }
  static std::string_view getPrefix() { return "OpenCL Error"; }
};

class EngineError : public Error {
public:
  explicit EngineError(std::string_view str) : Error({getPrefix(), str}) {}
  static std::string_view getPrefix() { return "OCL Engine Error"; }
};

} // namespace ocl