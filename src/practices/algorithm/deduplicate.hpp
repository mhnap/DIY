#pragma once

#include <algorithm>
#include <vector>

namespace algorithm {

template <typename T>
[[nodiscard]] auto deduplicateStd(const std::vector<T>& vec) {
  auto result = vec;
  result.erase(std::unique(result.begin(), result.end()), result.end());
  return result;
}

template <typename T>
[[nodiscard]] auto deduplicate(const std::vector<T>& vec) {
  auto result = vec;

  const auto last = result.size();
  decltype(last) first = 0;

  const auto adjacent_find = [&](auto first, auto last) {
    if (first == last) {
      return last;
    }
    auto next = first;
    while (++next != last) {
      if (result[first] == result[next]) {
        return first;
      }
      first = next;
    }
    return last;
  };

  const auto unique = [&](auto first, auto last) {
    // Skip the beginning, if already unique.
    first = adjacent_find(first, last);
    if (first == last) {
      return last;
    }

    // Do the real copy work.
    auto dest = first;
    ++first;
    while (++first != last) {
      if (result[dest] != result[first]) {
        result[++dest] = result[first];
      }
    }
    return ++dest;
  };

  result.resize(unique(first, last));
  return result;
}

} // namespace algorithm
