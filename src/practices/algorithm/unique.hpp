#pragma once

#include <algorithm>
#include <vector>

namespace algorithm::unique {

template <typename T>
[[nodiscard]] auto v1(const std::vector<T>& vec) {
  auto result = vec;

  const auto unique = [](auto first, auto last) {
    while (first != last) {
      last = std::remove(++first, last, *first);
    }
    return last;
  };

  result.erase(unique(result.begin(), result.end()), result.end());
  return result;
}

template <typename T>
[[nodiscard]] auto v2(const std::vector<T>& vec) {
  auto result = vec;

  const auto find_if = [](auto first, auto last, auto value) {
    while (first != last && *first != value) {
      ++first;
    }
    return first;
  };

  const auto remove_if = [find_if](auto first, auto last, auto value) {
    first = find_if(first, last, value);
    if (first == last) {
      return first;
    }
    auto result = first;
    ++first;
    for (; first != last; ++first) {
      if (*first != value) {
        *result = *first;
        ++result;
      }
    }
    return result;
  };

  auto unique = [remove_if](auto first, auto last) {
    while (first != last) {
      last = remove_if(++first, last, *first);
    }
    return last;
  };

  result.erase(unique(result.begin(), result.end()), result.end());
  return result;
}

} // namespace algorithm::unique
