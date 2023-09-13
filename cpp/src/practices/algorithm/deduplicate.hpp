#pragma once

#include <algorithm>
#include <tuple>
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

  const auto adjacent_find = [](auto first, const auto last) {
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

  const auto deduplicate = [adjacent_find](auto first, const auto last) {
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

  result.erase(deduplicate(result.begin(), result.end()), result.end());
  return result;
}

// Using vector with already filled data, but without adjacent_find
template <typename T>
[[nodiscard]] auto v3(const std::vector<T>& vec) {
  auto result = vec;

  const auto deduplicate = [](auto first, const auto last) {
    if (first == last) {
      return last;
    }
    // Do the real copy work.
    auto dest = first;
    while (++first != last) {
      if (*dest != *first) {
        *++dest = *first;
      }
    }
    return ++dest;
  };

  result.erase(deduplicate(result.begin(), result.end()), result.end());
  return result;
}

// Using empty vector
template <typename T>
[[nodiscard]] auto v4(const std::vector<T>& vec) {
  std::vector<T> result(vec.size());

  const auto last = result.size();
  decltype(last) first = 0;

  const auto deduplicate = [&result, &vec](auto first, const auto last) {
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

  result.resize(deduplicate(first, last));
  return result;
}

// Using empty vector and save metadata
template <typename T>
[[nodiscard]] auto v5(const std::vector<T>& vec) {
  std::vector<T> result(vec.size());
  std::vector<size_t> indices(vec.size());
  std::vector<size_t> revIndices(vec.size());
  std::vector<size_t> occurrences(vec.size());

  const auto last = result.size();
  decltype(last) first = 0;

  const auto deduplicate = [&](auto first, const auto last) {
    if (first == last) {
      return last;
    }
    auto dest = first;
    result[dest] = vec[first];
    indices[dest] = first;
    revIndices[first] = dest;
    ++occurrences[dest];
    while (++first != last) {
      if (result[dest] != vec[first]) {
        result[++dest] = vec[first];
        indices[dest] = first;
      }
      revIndices[first] = dest;
      ++occurrences[dest];
    }
    return ++dest;
  };

  auto end = deduplicate(first, last);

  result.resize(end);
  indices.resize(end);
  occurrences.resize(end);
  return std::tuple(result, indices, revIndices, occurrences);
}

} // namespace algorithm::deduplicate
