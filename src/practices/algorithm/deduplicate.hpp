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

  unsigned first = 0;
  const unsigned last = result.size();
  unsigned dest = first;

  while (++first != last) {
    if (result[dest] != result[first]) {
      result[++dest] = result[first];
    }
  }

  result.resize(++dest);
  return result;
}

} // namespace algorithm
