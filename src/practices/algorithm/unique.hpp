#pragma once

#include <algorithm>
#include <numeric>
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

  const auto find_if = [](auto first, const auto last, const auto value) {
    while (first != last && *first != value) {
      ++first;
    }
    return first;
  };

  const auto remove_if = [find_if](auto first, const auto last, const auto value) {
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

template <typename T>
[[nodiscard]] auto v3(const std::vector<T>& vec) {
  std::vector<size_t> indices(vec.size());
  std::iota(indices.begin(), indices.end(), 0);

  std::sort(indices.begin(), indices.end(), [&vec](auto a, auto b) { return vec[a] < vec[b]; });

  auto unique = [&vec](auto first, const auto last) {
    if (first == last) {
      return last;
    }
    // Do the real copy work.
    auto dest = first;
    *dest = *first;
    while (++first != last) {
      if (vec[*dest] != vec[*first]) {
        *++dest = *first;
      }
    }
    return ++dest;
  };

  indices.erase(unique(indices.begin(), indices.end()), indices.end());

  std::sort(indices.begin(), indices.end());

  std::vector<T> result(indices.size());
  for (size_t i = 0; i < indices.size(); ++i) {
    result[i] = vec[indices[i]];
  }
  return result;
}

} // namespace algorithm::unique
