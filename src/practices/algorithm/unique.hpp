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

template <typename T>
[[nodiscard]] auto v4(const std::vector<T>& vec) {
  std::vector<T> result;

  for (auto val : vec) {
    bool unique = true;
    for (auto res : result) {
      if (val == res) {
        unique = false;
      }
    }
    if (unique) {
      result.emplace_back(val);
    }
  }

  return result;
}

// Sorted with metadata
template <typename T>
[[nodiscard]] auto v5(const std::vector<T>& vec) {
  std::vector<T> result = vec;
  std::vector<size_t> indices(vec.size());
  std::iota(indices.begin(), indices.end(), 0);
  std::vector<size_t> revIndices(vec.size());
  std::vector<size_t> occurrences(vec.size());

  std::stable_sort(result.begin(), result.end(), [](auto a, auto b) { return a < b; });
  std::stable_sort(indices.begin(), indices.end(), [&vec](auto a, auto b) { return vec[a] < vec[b]; });

  const auto deduplicate = [&](auto first, const auto last) {
    if (first == last) {
      return last;
    }
    auto dest = first;
    revIndices[indices[first]] = dest;
    ++occurrences[dest];
    while (++first != last) {
      if (result[dest] != result[first]) {
        result[++dest] = result[first];
        indices[dest] = indices[first];
      }
      revIndices[indices[first]] = dest;
      ++occurrences[dest];
    }
    return ++dest;
  };

  const auto last = result.size();
  decltype(last) first = 0;
  auto end = deduplicate(first, last);

  result.resize(end);
  indices.resize(end);
  occurrences.resize(end);

  return std::tuple(result, indices, revIndices, occurrences);
}

// Unsorted with metadata
template <typename T>
[[nodiscard]] auto v6(const std::vector<T>& vec) {
  std::vector<T> result(vec.size());
  std::vector<size_t> indices(vec.size());
  std::vector<size_t> revIndices(vec.size());
  std::vector<size_t> occurrences(vec.size());

  const auto unique = [&](auto first, const auto last) {
    auto unique_length = 0U;
    for (; first != last; ++first) {
      const auto& val = vec[first];
      bool unique = true;
      for (auto unique_idx = 0U; unique_idx < unique_length; ++unique_idx) {
        const auto& unique_val = result[unique_idx];
        if (val == unique_val) {
          unique = false;
          revIndices[first] = unique_idx;
          ++occurrences[unique_idx];
          break;
        }
      }
      if (unique) {
        result[unique_length] = val;
        indices[unique_length] = first;
        revIndices[first] = unique_length;
        ++occurrences[unique_length];
        ++unique_length;
      }
    }
    return unique_length;
  };

  const auto last = result.size();
  decltype(last) first = 0;
  auto end = unique(first, last);

  result.resize(end);
  indices.resize(end);
  occurrences.resize(end);

  return std::tuple(result, indices, revIndices, occurrences);
}

} // namespace algorithm::unique
