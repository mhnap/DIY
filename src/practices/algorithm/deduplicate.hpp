#pragma once

#include <algorithm>
#include <vector>

namespace algorithm::deduplicate {

template <typename T>
[[nodiscard]] auto v1(const std::vector<T>& vec) {
  auto result = vec;
  result.erase(std::unique(result.begin(), result.end()), result.end());
  return result;
}

// Using vector with already filled data
template <typename T>
[[nodiscard]] auto v2(const std::vector<T>& vec) {
  auto result = vec;

  const auto adjacent_find = [](auto first, auto last) {
    if (first == last) {
      return last;
    }
    auto next = first;
    while (++next != last) {
      if (*first == *next) {
        return first;
      }
      first = next;
    }
    return last;
  };

  const auto unique = [adjacent_find](auto first, auto last) {
    // Skip the beginning, if already unique.
    first = adjacent_find(first, last);
    if (first == last) {
      return last;
    }
    // Do the real copy work.
    auto dest = first;
    ++first;
    while (++first != last) {
      if (*dest != *first) {
        *++dest = *first;
      }
    }
    return ++dest;
  };

  result.erase(unique(result.begin(), result.end()), result.end());
  return result;
}

// Using empty vector
template <typename T>
[[nodiscard]] auto v3(const std::vector<T>& vec) {
  std::vector<T> result(vec.size());

  const auto last = result.size();
  decltype(last) first = 0;

  const auto unique = [&result, &vec](auto first, auto last) {
    if (first == last) {
      return last;
    }
    auto dest = first;
    result[dest] = vec[first];
    while (++first != last) {
      if (result[dest] != vec[first]) {
        result[++dest] = vec[first];
      }
    }
    return ++dest;
  };

  result.resize(unique(first, last));
  return result;
}

} // namespace algorithm::deduplicate
